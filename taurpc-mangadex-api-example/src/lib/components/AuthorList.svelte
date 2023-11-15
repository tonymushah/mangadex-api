<script lang="ts">
	import { createTauRPCProxy, type ApiObject, type AuthorAttributes } from "$lib/mangadex_7";
	import { getName } from '@tauri-apps/api/app';
    import { Button } from "@svelteuidev/core";

	import Author from "./Author.svelte";

    let author_list : ApiObject<AuthorAttributes>[] = []

    let name = "";

    async function fetch_author() {
        const taurpc = await createTauRPCProxy();
        name = await getName();
        author_list = (await taurpc.mangadex_author.list({})).data;
    }
</script>

<Button on:click={fetch_author}>
Fetch Author
</Button>

{#each (author_list) as author}
    <Author {author}/>
{/each}

<p>Name : {name}</p>