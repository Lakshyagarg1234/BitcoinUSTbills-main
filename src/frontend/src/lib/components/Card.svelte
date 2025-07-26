<script lang="ts">
  import Card from "./ui/Card.svelte";

  export let principal = "";
  export let string = "";
</script>

<Card>
  <h3 data-testid="principal" slot="start">{principal}</h3>
  <p data-testid="string">{string}</p>
</Card>
