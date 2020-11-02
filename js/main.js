function getTime() {
	let date = new Date(),
        min = date.getMinutes(),
        hour = date.getHours();
    
        return '' + 
        	(hour < 10 ? ('0' + hour) : hour) + ':' + 
                (min < 10 ? ('0' + min) : min);
}

window.onload = () => {
	document.getElementById("clock").innerHTML = getTime();
	document.getElementById("search").focus();
        
	// focus prompt on any keypress
	let foc = document.body;
	foc.addEventListener('keydown', (event) => {
		document.getElementById("search").focus();
	});

        setInterval( () => {
        	document.getElementById("clock").innerHTML = getTime();
        },100);
}
