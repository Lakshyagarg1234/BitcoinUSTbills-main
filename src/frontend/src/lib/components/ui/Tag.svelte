<script lang="ts">
  export let intent = "default";
  export let testId = "";
</script>

<span class="tag tag-{intent}" data-testid={testId}>
  <slot></slot>
</span>

<style>
  .tag {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem 0.75rem;
    border-radius: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    margin: 0.5rem 0;
  }

  .tag-default {
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #d1d5db;
  }

  .tag-success {
    background: #dcfce7;
    color: #166534;
    border: 1px solid #bbf7d0;
  }

  .tag-error {
    background: #fecaca;
    color: #991b1b;
    border: 1px solid #fca5a5;
  }
</style>
