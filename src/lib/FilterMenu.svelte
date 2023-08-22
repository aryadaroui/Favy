<script lang="ts">
	import Heart from '$lib/icons/Heart.svelte';
	import Stars3 from '$lib/icons/Stars3.svelte';
	import Stars2 from '$lib/icons/Stars2.svelte';
	import Stars1 from '$lib/icons/Stars1.svelte';
	import Xcross from '$lib/icons/Xcross.svelte';
	import And from '$lib/icons/And.svelte';
	import Dot from '$lib/icons/Dot.svelte';
	import ButtonToggle from '$lib/ButtonToggle.svelte';

	let filter_settings = {
		dislike: false,
		favorite: false,
		and: false,
		star3: false,
		star2: false,
		star1: false,
	};
</script>

<div class="row">
	<ButtonToggle on_click={(is_on_after) => {}}>
		<Dot />
	</ButtonToggle>
	<ButtonToggle on_click={(is_on_after) => {}}>
		<Xcross />
	</ButtonToggle>
	<ButtonToggle on_click={(is_on_after) => {}}>
		<Heart />
	</ButtonToggle>
	<ButtonToggle on_click={(is_on_after) => {}}>
		<And />
	</ButtonToggle>
</div>

<div class="row">
	<ButtonToggle on_click={(is_on_after) => {}}>
		<Dot />
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
</div>

<style lang="scss">
	.row {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
	}
</style>
