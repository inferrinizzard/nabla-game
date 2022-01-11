import('../pkg/index.js')
	.then(module => {
		module.main_js();
		// module.game.test_new();
		console.log(module);
		console.log(new module.Game());
	})
	.catch(console.error);
