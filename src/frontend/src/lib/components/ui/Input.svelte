<script lang="ts">
  export let name: string;
  export let inputType = "text";
  export let value = "";
  export let disabled = false;
  export let placeholder = "";
</script>

<input
  {name}
  type={inputType}
  bind:value
  {disabled}
  {placeholder}
  class="input"
/>

<style>
  .input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.2s ease;
    background: white;
  }

  .input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .input:disabled {
    background: #f3f4f6;
    color: #9ca3af;
    cursor: not-allowed;
  }
</style>
