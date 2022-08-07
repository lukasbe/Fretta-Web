<script lang="ts">
    export let leftPercentage = 50;
</script>

<div class="two-column-container">
    <div class="left" style="--column-width: {leftPercentage}%"><slot name="left" /></div>
    <div class="right" style="--column-width: {100 - leftPercentage}%"><slot name="right" /></div>
</div>

<style lang="scss">
    .two-column-container {
        display: flex;
        > div{
            margin-inline: .5rem;
            flex: 0 1 var(--column-width);
        }
    }
</style>