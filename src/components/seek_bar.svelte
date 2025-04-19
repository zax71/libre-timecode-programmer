<!-- Accent is the already filled section -->
<input
  type="range"
  min="0"
  max="100000"
  value="1000"
  class="styled-slider w-full"
/>

<style>
  :root {
    --dot-radius: 1.7em;
    --bar-height: 0.4em;
  }
  /*generated with Input range slider CSS style generator (version 20211225)
https://toughengineer.github.io/demo/slider-styler*/
  input[type="range"].styled-slider {
    height: max(var(--dot-radius), var(--bar-height));
    appearance: none;
  }

  /*webkit*/
  input[type="range"].styled-slider::-webkit-slider-thumb {
    appearance: none;
    width: var(--dot-radius);
    height: var(--dot-radius);
    border-radius: var(--dot-radius);
    background: #e28010;

    margin-top: calc(var(--bar-height) * 0.5 - var(--dot-radius) * 0.5);
  }

  input[type="range"].styled-slider::-webkit-slider-runnable-track {
    height: var(--bar-height);
    background: #ffffff;
    border-radius: 0.3em;
  }
</style>
