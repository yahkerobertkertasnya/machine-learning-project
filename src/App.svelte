<script lang="ts">
  import { Route, Router } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import PreprocessDataset from "./routes/preprocess/PreprocessDataset.svelte";
  import Preprocess from "./routes/preprocess/Preprocess.svelte";
  import Training from "./routes/training/Training.svelte";
  import TrainingDataset from "./routes/training/TrainingDataset.svelte";
  import TestingDataset from "./routes/testing/TestingModel.svelte";
  import Testing from "./routes/testing/Testing.svelte";
  import Template from "./lib/components/Template.svelte";
</script>

<Router>
  <Route path="/">
    <Home />
  </Route>
  <Route path="/preprocess-dataset">
    <PreprocessDataset />
  </Route>
  <Route path="/training-dataset">
    <TrainingDataset />
  </Route>
  <Route path="/testing-dataset">
    <TestingDataset />
  </Route>
  <Route
    let:params
    path="/preprocess-dataset/:name">
    <Preprocess name={params.name} />
  </Route>
  <Route
    let:params
    path="/training-dataset/:name">
    <Training name={params.name} />
  </Route>
  <Route
    let:params
    path="/testing-dataset/:name">
    <Testing name={params.name} />
  </Route>
  <Route path="*">
    <Template />
  </Route>
</Router>
