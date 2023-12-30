<script lang="ts">
    export let size: string = "medium";
</script>

<button
    class:small = {size == "small"}
    class:medium = {size == "medium"}
    {... $$restProps}
>
    <slot />
</button>

<style>
    button {
        background-color: #90ee90;
        border: none;
        border-radius: 0.25em;
        color: var(--text-color);
        font-size: 2em;
        cursor: pointer;
        margin: 1em;
    }

    .small {
        padding: 0.25em 0.5em;
    }

    .medium {
        padding: 0.5em 1.5em;
    }

</style>