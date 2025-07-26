<div class="card">
  <slot name="start"></slot>
  <slot></slot>
</div>

<style>
  .card {
    background: white;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
  }

  .card :global(h3) {
    margin: 0 0 0.5rem 0;
    color: #2d3748;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .card :global(p) {
    margin: 0;
    color: #4a5568;
    line-height: 1.5;
  }
</style>
