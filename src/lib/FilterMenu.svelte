<script lang="ts">
	import Heart from '$lib/icons/Heart.svelte';
	import Stars3 from '$lib/icons/Stars3.svelte';
	import Stars2 from '$lib/icons/Stars2.svelte';
	import Stars1 from '$lib/icons/Stars1.svelte';
	import Xcross from '$lib/icons/Xcross.svelte';
	import And from '$lib/icons/And.svelte';
	import ButtonToggle from '$lib/ButtonToggle.svelte';
</script>

<p>Filter menu stuff!</p>

<ButtonToggle on_click={(is_on_after) => {}}>
	<And />
</ButtonToggle>
<ButtonToggle on_click={(is_on_after) => {}}>
	<Xcross />
</ButtonToggle>
<ButtonToggle on_click={(is_on_after) => {}}>
	<Stars1 />
</ButtonToggle>
<ButtonToggle on_click={(is_on_after) => {}}>
	<Stars2 />
</ButtonToggle>
<ButtonToggle on_click={(is_on_after) => {}}>
	<Stars3 />
</ButtonToggle>
<ButtonToggle on_click={(is_on_after) => {}}>
	<Heart />
</ButtonToggle>


<style lang="scss">
</style>
