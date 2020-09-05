class Dashboard extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      metrics: {
        videos: []
      },
      lastFetch: 0
    };
    this.getUpdate = this.getUpdate.bind(this);
  }

  getUpdate() {
    const dash = this;
    $.ajax("/api/metrics", {
      method: "GET"
    }).done(function (msg, textStatus, xhrObj) {
      dash.setState({
        metrics: msg,
        lastFetch: Math.floor(Date.now() / 1000)
      });
    }).always(function () {
      setTimeout(dash.getUpdate, 1000);
    });
  }

  render() {
    let totalActiveStreams = 0;
    let totalDistinctVideosBeingWatched = 0;
    let videosBeingWatchedByEachIp = {};
    const displayLastUpdated = new Date(this.state.lastFetch * 1000).toUTCString();
    let vids = [];
    this.state.metrics.videos.forEach(function (singleVideoMetrics, index) {
      const watchers = Object.keys(singleVideoMetrics.watchers).length;
      totalActiveStreams += watchers;

      if (watchers > 0) {
        ++totalDistinctVideosBeingWatched;
      }

      Object.keys(singleVideoMetrics.watchers).forEach(function (ip, watcherIndex) {
        if (!videosBeingWatchedByEachIp.hasOwnProperty(ip)) {
          videosBeingWatchedByEachIp[ip] = [];
        }

        videosBeingWatchedByEachIp[ip].push(index);
      });
      vids.push( /*#__PURE__*/React.createElement(VideoMetrics, {
        metrics: singleVideoMetrics,
        videoName: videos[index]
      }));
    });
    const totalDistinctIPsWatchingVideos = Object.keys(videosBeingWatchedByEachIp).length;
    return /*#__PURE__*/React.createElement("div", {
      className: "Dashboard"
    }, /*#__PURE__*/React.createElement("dl", null, /*#__PURE__*/React.createElement("dt", null, "Active streams"), /*#__PURE__*/React.createElement("dd", null, totalActiveStreams), /*#__PURE__*/React.createElement("dt", null, "Distinct videos being watched"), /*#__PURE__*/React.createElement("dd", null, totalDistinctVideosBeingWatched), /*#__PURE__*/React.createElement("dt", null, "Distinct IPs watching videos"), /*#__PURE__*/React.createElement("dd", null, totalDistinctIPsWatchingVideos), /*#__PURE__*/React.createElement("dt", null, "This display last updated"), /*#__PURE__*/React.createElement("dd", null, displayLastUpdated)), vids);
  }

}

class VideoMetrics extends React.Component {
  constructor(props) {
    super(props);
  }

  render() {
    const watcherMap = this.props.metrics.watchers;
    const ips = Object.keys(watcherMap);
    const displayCount = ips.length > 0 ? ips.length + " viewer(s)" : "";
    let watchers = [];
    ips.forEach(function (ip, index) {
      watchers.push( /*#__PURE__*/React.createElement(Watcher, {
        ip: ip,
        percent: watcherMap[ip].percent
      }));
    });
    return /*#__PURE__*/React.createElement("fieldset", {
      className: "CountrySearchForm"
    }, /*#__PURE__*/React.createElement("legend", null, this.props.videoName), displayCount, /*#__PURE__*/React.createElement("br", null), watchers);
  }

}

function Watcher(props) {
  return /*#__PURE__*/React.createElement("div", {
    className: "CountryListItem"
  }, props.ip, /*#__PURE__*/React.createElement("br", null), props.percent, "%");
}