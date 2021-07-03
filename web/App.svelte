<script>
	import { Router, Route, Link } from "yrv";
	import Table from "./SomePage.svelte";
</script>

<Link href="/">Home</Link>
| <Link href="#World">Hello</Link>

<p>
	<Router>
		<Route exact>Hello world</Route>
		<Route exact path="#World">
			<Table />
		</Route>
	</Router>
</p>
