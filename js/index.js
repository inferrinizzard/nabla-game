import('../pkg/index.js')
	.then(module => {
		// module.main_js();
		console.log(new module.Game());
	})
	.catch(console.error);
