<script lang="ts">
    import Button from "$lib/button.svelte";
</script>


<body>
    <div class="container">
        <Button size = "medium">
            ➕ Create
        </Button>
        <Button size = "medium">
            ❗Clone
        </Button>
        <Button size = "medium">
            📂 Open
        </Button>
    </div>
</body>

<style>
    .container {
    margin-top: 250px;
    margin-left: 35%;
    max-width: 500px;
    display: grid;
    align-items: center;
    }
</style>