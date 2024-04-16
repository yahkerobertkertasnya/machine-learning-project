 // This file has been generated by Specta. DO NOT EDIT.

export type Dataset = { name: string; data_amount: number }

export type TauRpcDatasetApiInputTypes = { proc_name: "get_datasets"; input_type: null } | { proc_name: "get_dataset_thumbnail"; input_type: { __taurpc_type: string } }

export type TauRpcDatasetApiOutputTypes = { proc_name: "get_datasets"; output_type: Dataset[] } | { proc_name: "get_dataset_thumbnail"; output_type: number[] }

export type TauRpcUtilApiInputTypes = { proc_name: "get_current_dir"; input_type: null }

export type TauRpcUtilApiOutputTypes = { proc_name: "get_current_dir"; output_type: string }

const ARGS_MAP = {"dataset":"{\"get_dataset_thumbnail\":[\"dataset_name\"],\"get_datasets\":[]}","util":"{\"get_current_dir\":[]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'dataset': [TauRpcDatasetApiInputTypes, TauRpcDatasetApiOutputTypes],
	'util': [TauRpcUtilApiInputTypes, TauRpcUtilApiOutputTypes],
}