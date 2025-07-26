<script lang="ts">
  import Input from "./ui/Input.svelte";
  import Tag from "./ui/Tag.svelte";
  import IconCheck from "./ui/IconCheck.svelte";

  // api function
  export let set: (name: string) => Promise<void>;

  // Form input
  let input: string = "";

  // Visibility
  let inputDisabled = false;
  let successVisible = false;

  const submit = async () => {
    inputDisabled = true;

    await set(input);
    successVisible = true;

    input = "";
    inputDisabled = false;
  };
</script>

<Input
  name="input"
  inputType="text"
  bind:value={input}
  disabled={inputDisabled}
  placeholder="Enter your name"
/>

<button data-testid="button" class="primary" on:click={submit} type="button"
  >Send</button
>

{#if successVisible}
  <Tag intent="success" testId="success">
    <IconCheck />
    Form successfully submitted!
  </Tag>
{/if}

<style>
  button.primary {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s ease;
    margin: 1rem 0;
  }

  button.primary:hover {
    background: #2563eb;
  }

  button.primary:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }
</style>
