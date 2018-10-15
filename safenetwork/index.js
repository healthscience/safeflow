// Usage Example
const safeApp = require('@maidsafe/safe-node-app');

console.log(safeApp);
// starting initialisation
let prms = safeApp.initializeApp({
                     id: "net.heealthscience.app",
                     name: 'HealthScience App',
                     vendor: 'healthscience.network.'
                     //customExecPath: isDevMode ? [`${process.execPath}`, `${app.getAppPath()}`] : [app.getPath('exe')]
                    });
// we want read-append access to `_pictures` and
// read access to `_videos`:
const containers = { '_videos': ['Read'], '_pictures' : ['Read', 'Insert']}
prms.then(app => app.auth.genAuthUri(containers
          ).then(uri => app.auth.openUri(uri)
       // now we either quit the programm
       // or wait for a result url
       ))
