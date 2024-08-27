// export multiple functions
import { emit, listen } from "@tauri-apps/api/event";

export default {
  // Tools
  getObjChanges: (obj1, obj2) =>
    Object.keys(obj1).reduce((acc, key) => {
      if (obj2 && obj2[key] !== obj1[key]) {
        acc[key] = obj1[key];
      }
      return acc;
    }, {}),

  // TDOD: should be in a separate file
  isFolder: (path) => path.endsWith("/"),
  isFile: (path) => !path.endsWith("/"),

  // Api inport and export

  requestImportXml: (xml_path) => {
    console.log(` ? => Import Xml: ${xml_path}`);
    emit("import-xml", {
      XmlPath: {
        xml_path: xml_path,
      },
    });
  },

  requestOpenFileDialog: () => {
    console.log(" ? => Open File Dialog");
    return new Promise((resolve, reject) => {
      listen("receive-file-path", (data) => {
        resolve(data.payload);
      });
      emit("open-file-dialog", {
        FileDialog: {
          title: "Open File",
          filters: [
            {
              name: "XML",
              extensions: ["xml"],
            },
          ],
        },
      });
    });
  },

  // Api POST
  requestUpdateConfig: (changes) => {
    for (let key in changes) {
      console.log(` ? => Update Config: ${key} = ${changes[key]}`);
      emit("update-config", {
        KeyAndValue: {
          key: key,
          value: changes[key].toString(),
        },
      });
    }
  },

  requestMoveNode: (source_node_path, target_node_path) => {
    console.log(` ? => Move Node: ${source_node_path} -> ${target_node_path}`);
    emit("move-node", {
      SourcePathAndTargetPath: {
        source_path: source_node_path,
        target_path: target_node_path,
      },
    });
  },

  addNodeToTrack: (track_path, node_path) => {
    console.log(` => Add Node to Track: ${node_path} -> ${track_path}`);
    emit("add-node-to-track", {
      NodePathAndTrackPath: {
        node_path: node_path,
        track_path: track_path,
      },
    });
  },

  addTracksWithNode: (node_path, track_paths) => {
    console.log(` => Add Tracks with Node: ${node_path} -> ${track_paths}`);
    for (let track_path of track_paths) {
      emit("add-track-with-node", {
        NodePathAndTrackPath: {
          node_path: node_path,
          track_path: track_path,
        },
      });
    }
  },

  addTrackWithNode: (node_path, track_path) => {
    console.log(` => Add Track with Node: ${node_path} -> ${track_path}`);
    emit("add-track-with-node", {
      NodePathAndTrackPath: {
        node_path: node_path,
        track_path: track_path,
      },
    });
  },

  // Api Requests
  requestNodes: () => {
    console.log(" ? => Request Nodes");
    emit("request-nodes");
  },

  requestNode: (node_path) => {
    console.log(` ? => Request Node: ${node_path}`);
    emit("request-node", {
      NodePath: {
        node_path: node_path,
      },
    });
  },

  requestConfig: () => {
    console.log(" ? => Request Config");
    emit("request-config");
  },

  requestReload: () => {
    console.log(" ? => Request Reload");
    emit("request-reload");
  },

  requestSoftReload: () => {
    console.log(" ? => Request Soft Reload");
    emit("request-soft-reload");
  },

  requestSave: () => {
    console.log(" ? => Request Save");
    emit("request-save");
  },

  // Api GET
  onNodesReceived(callback) {
    listen("receive-nodes", (data) => {
      console.log(" <= Nodes Received");
      callback(JSON.parse(data.payload));
    });
  },

  onNodeReceived(callback) {
    listen("receive-node", (data) => {
      console.log(" <= Node Received");
      callback(JSON.parse(data.payload));
    });
  },

  onConfigReceived(callback) {
    listen("receive-config", (data) => {
      console.log(" <= Config Received");
      callback(JSON.parse(data.payload));
    });
  },

  // Tauri Window

  onFileDropHover(callback) {
    listen("tauri://file-drop-hover", (event) => {
      console.log(`file-drop-hover: ${event.payload}`);
      callback(event.payload);
    });
  },

  onFileDrop(callback) {
    listen("tauri://file-drop", (event) => {
      console.log(`file-drop: ${event.payload}`);
      callback(event.payload);
    });
  },

  onFileDropCancel(callback) {
    listen("tauri://file-drop-cancelled", (event) => {
      console.log(`file-drop-cancelled: ${event.payload}`);
      callback(event.payload);
    });
  },
};

// Path: src\app\api\index.jsx


