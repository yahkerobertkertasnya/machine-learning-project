use crate::dataset::dataset::Dataset;

#[taurpc::procedures(path = "dataset")]
pub trait DatasetApi {
    async fn get_datasets() -> Result<Vec<Dataset>, String>;
    async fn get_dataset_thumbnail(dataset_name: String) -> Result<Vec<u8>, String>;
}