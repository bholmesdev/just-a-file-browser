<script>
  import '../styles/reset.css';

  let { children } = $props();
</script>

<svelte:boundary>
  {@render children()}

  {#snippet failed(error, reset)}
    <p>Unexpectedly goofed up!</p>
    <details>
      <summary>Scary details</summary>
      <p>{error instanceof Error ? error.message : 'Unknown error'}</p>
    </details>
    <button onclick={reset}>Try reloading</button>
  {/snippet}
</svelte:boundary>

<style>
  @font-face {
    font-family: 'Satoshi';
    src:
      url('/fonts/Satoshi-Variable.woff2') format('woff2'),
      url('/fonts/Satoshi-Variable.woff') format('woff'),
      url('/fonts/Satoshi-Variable.ttf') format('truetype');
    font-weight: 300 900;
    font-display: swap;
    font-style: normal;
  }
  @font-face {
    font-family: 'Satoshi-Italic';
    src:
      url('/fonts/Satoshi-VariableItalic.woff2') format('woff2'),
      url('/fonts/Satoshi-VariableItalic.woff') format('woff'),
      url('/fonts/Satoshi-VariableItalic.ttf') format('truetype');
    font-weight: 300 900;
    font-display: swap;
    font-style: italic;
  }

  :root {
    font-family: Satoshi;
    font-size: 15px;
    color: #121212;
  }
</style>
