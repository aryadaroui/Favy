<script lang="ts">
import {status} from '$lib/stores'

let progress_shown = false


$: $status.text, status_update()

function status_update() {
	progress_shown = false
	console.log('status_update', $status.text)
}

</script>




<style lang="scss">


</style>