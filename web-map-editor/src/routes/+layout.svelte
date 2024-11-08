<script>
  import "../app.css";
  import Header from "../components/header/header.svelte";
</script>

<div class="app">
  <Header />

  <main>
    <slot />
  </main>
</div>

<style></style>
