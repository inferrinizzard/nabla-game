import('../pkg/index.js')
	.then(module => {
		// module.main_js();
		let game = new module.Game();

		console.log(game);
	})
	.catch(console.error);
