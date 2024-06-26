 // This file has been generated by Specta. DO NOT EDIT.

export type ClassificationReport = { class: { [key in string]: Metrics }; accuracy: number; macro_avg: Metrics; weighted_avg: Metrics }

export type Dataset = { name: string; labels: Label[] }

export type GeneralDataset = { name: string; label_amount: number; data_amount: number }

export type Hyperparameters = { Svm: SVMHyperparameter } | { Knn: KNNHyperparameter } | { Lr: LRHyperparameter }

export type KNNHyperparameter = { n_neighbors: string; algorithm: string; weights: string; metric: string }

export type LRHyperparameter = { penalty: string; C: string; solver: string; max_iter: string }

export type Label = { name: string; data: string[]; is_preprocessed: boolean }

export type Metrics = { precision: number; recall: number; "f1-score": number; support: number }

export type Model = { name: string; model_specification: ModelSpecification; confusion_matrix_image: string }

export type ModelPrediction = { class: string; image_result: string }

export type ModelSpecification = { dataset_name: string; accuracy: number; precision: number; recall: number; f1: number; confusion_matrix: number[][]; hyperparameters: Hyperparameters; classification_report: ClassificationReport }

export type ProgressPayload = { name: string; label: string; current_amount: number; total_amount: number }

export type SVMHyperparameter = { kernel: string; C: string; gamma: string; degree: string }

export type TauRpcDatasetApiInputTypes = { proc_name: "get_all"; input_type: null } | { proc_name: "get_all_training_dataset"; input_type: null } | { proc_name: "get_all_testing_dataset"; input_type: null } | { proc_name: "get_random_image"; input_type: { __taurpc_type: string } } | { proc_name: "get_random_processed_image"; input_type: { __taurpc_type: string } } | { proc_name: "get_labels"; input_type: { __taurpc_type: string } } | { proc_name: "get_data"; input_type: [string, string] } | { proc_name: "get"; input_type: { __taurpc_type: string } } | { proc_name: "preprocess"; input_type: { __taurpc_type: string } } | { proc_name: "get_image"; input_type: [string, string, string] } | { proc_name: "get_processed_image"; input_type: [string, string, string] } | { proc_name: "get_processed_graphs"; input_type: { __taurpc_type: string } }

export type TauRpcDatasetApiOutputTypes = { proc_name: "get_all"; output_type: GeneralDataset[] } | { proc_name: "get_all_training_dataset"; output_type: TrainingDataset[] } | { proc_name: "get_all_testing_dataset"; output_type: TestingDataset[] } | { proc_name: "get_random_image"; output_type: string } | { proc_name: "get_random_processed_image"; output_type: string } | { proc_name: "get_labels"; output_type: Label[] } | { proc_name: "get_data"; output_type: string[] } | { proc_name: "get"; output_type: Dataset } | { proc_name: "preprocess"; output_type: null } | { proc_name: "get_image"; output_type: string } | { proc_name: "get_processed_image"; output_type: string | null } | { proc_name: "get_processed_graphs"; output_type: { [key in string]: string } }

export type TauRpcModelApiInputTypes = { proc_name: "train"; input_type: [string, string, string, Hyperparameters] } | { proc_name: "get_all"; input_type: null } | { proc_name: "get"; input_type: { __taurpc_type: string } } | { proc_name: "remove"; input_type: { __taurpc_type: string } } | { proc_name: "predict"; input_type: [string, string] }

export type TauRpcModelApiOutputTypes = { proc_name: "train"; output_type: null } | { proc_name: "get_all"; output_type: Model[] } | { proc_name: "get"; output_type: Model } | { proc_name: "remove"; output_type: null } | { proc_name: "predict"; output_type: ModelPrediction }

export type TauRpcUtilApiInputTypes = { proc_name: "get_current_dir"; input_type: null } | { proc_name: "open_directory"; input_type: null }

export type TauRpcUtilApiOutputTypes = { proc_name: "get_current_dir"; output_type: string } | { proc_name: "open_directory"; output_type: null }

export type TestingDataset = { name: string; dataset_name: string; accuracy: number }

export type TrainingDataset = { name: string; data_amount: number; feature_count: number }

const ARGS_MAP = {"model":"{\"get\":[\"model_name\"],\"predict\":[\"model_name\",\"image\"],\"train\":[\"dataset_name\",\"model_name\",\"algorithm\",\"hyperparameter\"],\"remove\":[\"model_name\"],\"get_all\":[]}","dataset":"{\"get_all\":[],\"get_data\":[\"dataset_name\",\"label_name\"],\"preprocess\":[\"dataset_name\"],\"get_processed_graphs\":[\"name\"],\"get_all_training_dataset\":[],\"get_labels\":[\"dataset_name\"],\"get_all_testing_dataset\":[],\"get\":[\"dataset_name\"],\"get_image\":[\"name\",\"label\",\"data\"],\"get_processed_image\":[\"name\",\"label\",\"data\"],\"get_random_image\":[\"path\"],\"get_random_processed_image\":[\"path\"]}","util":"{\"get_current_dir\":[],\"open_directory\":[]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'dataset': [TauRpcDatasetApiInputTypes, TauRpcDatasetApiOutputTypes],
	'util': [TauRpcUtilApiInputTypes, TauRpcUtilApiOutputTypes],
	'model': [TauRpcModelApiInputTypes, TauRpcModelApiOutputTypes],
}