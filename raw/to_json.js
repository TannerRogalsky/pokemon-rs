const fs = require('fs');
const files = [
	["learnsets", "BattleLearnsets"], 
	["pokedex", "BattlePokedex"],
	["moves", "BattleMovedex"],
	["typechart", "BattleTypeChart"],
];

try {

	for (const data of files) {
		let file = data[0];
		let prop = data[1];
		let t = require(`./${file}.js`)[prop];
		fs.writeFileSync(`./${file}.json`, JSON.stringify(t, null, 2));
	}
    
} catch(e) {
    console.log('Error:', e.stack);
}