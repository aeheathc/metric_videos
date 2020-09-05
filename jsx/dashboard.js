class Dashboard extends React.Component
{
    constructor(props)
    {
        super(props);
        this.state = {metrics: {videos: []}, lastFetch: 0};
        this.getUpdate = this.getUpdate.bind(this);
    }

    getUpdate()
    {
        const dash = this;
        $.ajax("/api/metrics", {method: "GET"})
            .done( function(msg, textStatus, xhrObj) 
            {
                dash.setState({metrics: msg, lastFetch: Math.floor(Date.now() / 1000)});
            })
            .always( function()
            {
                setTimeout(dash.getUpdate, 1000);
            })
    }

    render()
    {
        let totalActiveStreams = 0;
        let totalDistinctVideosBeingWatched = 0;
        let videosBeingWatchedByEachIp = {};
        const displayLastUpdated = new Date(this.state.lastFetch * 1000).toUTCString();

        let vids=[];
        this.state.metrics.videos.forEach(function(singleVideoMetrics,index)
        {
            const watchers = Object.keys(singleVideoMetrics.watchers).length;
            totalActiveStreams += watchers;
            if(watchers > 0) {++totalDistinctVideosBeingWatched;}
            
            Object.keys(singleVideoMetrics.watchers).forEach(function(ip, watcherIndex)
            {
                if(!videosBeingWatchedByEachIp.hasOwnProperty(ip)) {videosBeingWatchedByEachIp[ip] = [];}
                videosBeingWatchedByEachIp[ip].push(index);
            });

            vids.push  (
                <VideoMetrics metrics={singleVideoMetrics} videoName={videos[index]}/>
            );
        });

        const totalDistinctIPsWatchingVideos = Object.keys(videosBeingWatchedByEachIp).length;

        return (
            <div className="Dashboard">
             <dl>
              <dt>Active streams</dt>
              <dd>{totalActiveStreams}</dd>
              <dt>Distinct videos being watched</dt>
              <dd>{totalDistinctVideosBeingWatched}</dd>
              <dt>Distinct IPs watching videos</dt>
              <dd>{totalDistinctIPsWatchingVideos}</dd>
              <dt>This display last updated</dt>
              <dd>{displayLastUpdated}</dd>
             </dl>
             {vids}
            </div>
        );
    }
}

class VideoMetrics extends React.Component
{
    constructor(props)
    {
        super(props);
    }

    render()
    {
        const watcherMap = this.props.metrics.watchers;

        const ips = Object.keys(watcherMap);
        const displayCount = (ips.length > 0) ? (ips.length + " viewer(s)") : "";
        let watchers = [];
        ips.forEach(function(ip,index)
        {
            watchers.push(
                <Watcher ip={ip} percent={watcherMap[ip].percent}/>
            );
        });

        return (
            <fieldset className="CountrySearchForm">
             <legend>{this.props.videoName}</legend>
             {displayCount}<br/>{watchers}
            </fieldset>
        );
    }
}



function Watcher(props)
{
    return (
        <div className="CountryListItem">
         {props.ip}<br/>{props.percent}%
        </div>
    );
}
