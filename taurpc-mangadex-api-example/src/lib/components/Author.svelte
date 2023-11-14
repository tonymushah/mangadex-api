<script lang="ts">
    import type { ApiData, ApiObject, AuthorAttributes } from "$lib/bindings";
    type Author = ApiObject<AuthorAttributes>;
    export let author: Author;
    let name = author.attributes.name;
    let works = author.relationships.filter((rel) => rel.type == "manga").length;
</script>

<div>
    <p>Name : {name}</p>
    <p>Works count: {works}</p>
</div>