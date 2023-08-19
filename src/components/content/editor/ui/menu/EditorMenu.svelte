<script>
	import { EditorMenu } from "../../styles/EditorMenu.style";
    import ArrowLeft from "svelte-material-icons/ArrowLeft.svelte"

    function return_main() {
        window.location.href = "/"
    }

</script>
<header class={EditorMenu}>
    <section>
        <span role="button" tabindex="0" on:click={return_main} on:keypress={return_main}>
            <ArrowLeft/>
            <span>
                Voltar
            </span>
        </span>
    </section>
</header>