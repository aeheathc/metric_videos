'use strict';

let currentVid = -1;
$(report);

/* Switch the player to another video by index.
*/
function switchVid(vidId)
{
    currentVid = vidId;
    $("#player").attr("src",videos[vidId]);
}

//send information about current video to server
function report()
{
    if(currentVid > -1)
    {
        const player = $("#player");
        let duration = player.prop("duration");
        let currentTime = player.prop("currentTime");
        if(isNaN(duration)) {duration = 1;}
        if(isNaN(currentTime)) {currentTime = 0;}
        const percent = Math.floor((currentTime / duration) * 100);
        const endpoint = "/api/watcher/" + currentVid + '/' + percent;
        $.ajax(endpoint, {method: "POST"});
    }
    setTimeout(report, 1000);
}
