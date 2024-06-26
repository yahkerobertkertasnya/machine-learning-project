use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use crate::constants::{CONFUSION_MATRIX_IMAGE, KNN_TRAIN_SCRIPT, LR_TRAIN_SCRIPT, MODEL_DIRECTORY, MODEL_SPECIFICATION_JSON, PREDICT_SCRIPT, PROCESSED_DIRECTORY, PROCESSED_OUTPUT_CSV, SCRIPTS_DIRECTORY, SVM_TRAIN_SCRIPT, TEMP_DIRECTORY, TESTING_INPUT_IMAGE, TESTING_OUTPUT_IMAGE};
use crate::model::api::ModelApi;
use crate::model::model::{Hyperparameters, Model, ModelPrediction, ModelSpecification};
use crate::py_utils::run_script;
use crate::utils::{FileType, get_directory_content, read_file, remove_directory_content, write_file};

#[derive(Clone)]
pub struct ModelApiImpl;

#[taurpc::resolvers]
impl ModelApi for ModelApiImpl {
    async fn train(
        self,
        dataset_name: String,
        model_name: String,
        algorithm: String,
        hyperparameter: Hyperparameters,
    ) -> Result<(), String> {
        let csv = Path::new(PROCESSED_DIRECTORY)
            .join(dataset_name)
            .join(PROCESSED_OUTPUT_CSV);

        let file = File::open(&csv);

        if file.is_err() {
            return Err("Dataset not found".to_string());
        }

        let dataset_path = csv.to_str().unwrap().to_string();


        let script_path = match algorithm.as_str() {
            "lr" => Path::new(SCRIPTS_DIRECTORY).join(LR_TRAIN_SCRIPT),
            "knn" => Path::new(SCRIPTS_DIRECTORY).join(KNN_TRAIN_SCRIPT),
            "svm" => Path::new(SCRIPTS_DIRECTORY).join(SVM_TRAIN_SCRIPT),
            _ => return Err("Model type not found".to_string()),
        };

        let mut model_args = vec![
            dataset_path,
            model_name,
        ];

        match hyperparameter {
            Hyperparameters::Svm(hyperparameter) => {
                model_args.extend(vec![
                    hyperparameter.kernel,
                    hyperparameter.c,
                    hyperparameter.gamma,
                    hyperparameter.degree,
                ]);
            }
            Hyperparameters::Knn(hyperparameter) => {
                model_args.extend(vec![
                    hyperparameter.n_neighbors,
                    hyperparameter.algorithm,
                    hyperparameter.weights,
                    hyperparameter.metric,
                ]);
            }
            Hyperparameters::Lr(hyperparameter) => {
                model_args.extend(vec![
                    hyperparameter.penalty,
                    hyperparameter.c,
                    hyperparameter.solver,
                    hyperparameter.max_iter,
                ]);
            }
        }


        let mut child = run_script(
            &script_path,
            model_args,
        );
        
        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
        
        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }

        let status = child.wait().expect("failed to wait on child");

        if !status.success() {
            return Err("Failed to train model".to_string());
        }

        println!("Model trained");
        Ok(())
    }

    async fn get_all(self) -> Result<Vec<Model>, String> {
        let models_dir = Path::new(MODEL_DIRECTORY);

        let models = get_directory_content(models_dir, &FileType::Directory);

        let model_list = models
            .par_iter()
            .map(|model_dir| {
                let specification_json = model_dir.join(MODEL_SPECIFICATION_JSON);

                let mut file = File::open(specification_json).expect("Failed to open specification.json");
                let mut data = String::new();

                file.read_to_string(&mut data)
                    .expect("Failed to read specification.json");

                let model_name = model_dir
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let model: ModelSpecification =
                    serde_json::from_str(&data).expect("Failed to parse specification.json");

                let confusion_matrix_image = model_dir.join(CONFUSION_MATRIX_IMAGE);
                
                println!("{:?}", confusion_matrix_image);

                let image = read_file(&confusion_matrix_image).unwrap_or_default();

                Model {
                    name: model_name,
                    model_specification: model,
                    confusion_matrix_image: image,
                }
            })
            .collect::<Vec<Model>>();

        Ok(model_list)
    }

    async fn get(self, model_name: String) -> Result<Model, String> {
        let model_dir = Path::new(MODEL_DIRECTORY).join(&model_name);

        let specification_json = model_dir.join(MODEL_SPECIFICATION_JSON);

        let mut file = File::open(specification_json).expect("Failed to open specification.json");
        let mut data = String::new();

        file.read_to_string(&mut data)
            .expect("Failed to read specification.json");

        let model: ModelSpecification =
            serde_json::from_str(&data).expect("Failed to parse specification.json");

        let confusion_matrix_image = model_dir.join(CONFUSION_MATRIX_IMAGE);

        println!("{:?}", confusion_matrix_image);
        let image = read_file(&confusion_matrix_image).unwrap_or_default();

        let model = Model {
            name: model_name,
            model_specification: model,
            confusion_matrix_image: image,
        };

        Ok(model)
    }

    async fn remove(self, model_name: String) -> Result<(), String> {
        let model_dir = Path::new(MODEL_DIRECTORY).join(&model_name);

        remove_directory_content(&model_dir);

        Ok(())
    }

    async fn predict(self, model_name: String, image: String) -> Result<ModelPrediction, String> {
        let model_dir = Path::new(MODEL_DIRECTORY)
            .join(&model_name)
            .to_str()
            .unwrap()
            .to_string();

        let input_dir = Path::new(TEMP_DIRECTORY).join(TESTING_INPUT_IMAGE);
        let image = general_purpose::STANDARD.decode(image).unwrap();
        write_file(&input_dir, image);

        let script_path = Path::new(SCRIPTS_DIRECTORY).join(PREDICT_SCRIPT);
        let mut child = run_script(&script_path, vec![model_dir]);
        
        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
        
        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        let mut class = String::new();

        for line in reader.lines() {
            class = line.unwrap();
        }

        let output_dir = Path::new(TEMP_DIRECTORY).join(TESTING_OUTPUT_IMAGE);

        let image_result = read_file(&output_dir).unwrap();

        let prediction = ModelPrediction {
            class,
            image_result,
        };
        
        let status = child.wait().expect("failed to wait on child");

        if !status.success() {
            return Err("Failed to predict image".to_string());
        }

        Ok(prediction)
    }
}
