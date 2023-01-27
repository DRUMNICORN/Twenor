/**
 * @file Interface.tsx
 * @description Interface for connection to backend
 *
 */

// it uses tauri emit and listen to communicate with backend
import { emit, listen } from "@tauri-apps/api/event";
import { BackendNode, Property } from "./components/util/Datatypes";

// type DataTypes = number | string | boolean | null;

// make every value a string
function clearify_data_type(value: any) {
  if (value === null)
    return {
      None: null,
    };

  if (value === undefined)
    return {
      None: null,
    };

  if (typeof value === "number")
    if (value % 1 !== 0)
      // if float
      return {
        F32: value,
      };
    else
      return {
        I32: value,
      };

  if (typeof value === "boolean")
    return {
      Bool: value,
    };

  if (typeof value === "string")
    return {
      String: value,
    };

  if (typeof value === "object")
    return {
      Object: JSON.stringify(value),
    };

  console.log(`Unknown data type: ${typeof value}`);
}

const Interface = {
  reload: async (): Promise<any> => {
    return new Promise((resolve, _reject) => {
      listen(`RELOAD`, (event) => {
        resolve(event.payload);
      });

      emit(`REQUEST_RELOAD`);
    });
  },

  save_state: async (state: Object): Promise<any> => {
    return new Promise((resolve, _reject) => {
      listen(`STATE_SAVED`, (event) => {
        resolve(event.payload);
      });

      let states: Array<any> = [];
      Object.keys(state).forEach((key: string) => {
        let value: any = state[key as keyof typeof state];
        if (value != null && value != undefined && value != "") {
          states.push({
            key: key,
            value: clearify_data_type(value),
          });
        }
      });

      let data = {
        State: states,
      };

      let stringified = JSON.stringify(data);
      emit(`SAVE_STATE`, stringified);
    });
  },

  load_state: async (): Promise<Array<Property>> => {
    return new Promise((resolve, _reject) => {
      listen(`RECEIVE_STATE`, (event) => {
        let data: Array<Property> = [];
        let states = JSON.parse(event.payload as string) as Array<Property>;

        states.forEach((element: Property) => {
          let value_string = JSON.stringify(element.value);
          let value: any = value_string;

          if (element.value.String != undefined) value = element.value.String;
          if (element.value.I32 != undefined) value = element.value.I32;
          if (element.value.F32 != undefined) value = element.value.F32;
          if (element.value.Bool != undefined) value = element.value.Bool;
          if (element.value.Object != undefined) {
            let obj = element.value.Object;
            console.log(obj);
            value = JSON.parse(obj);
            console.log(value);
          }
          if (element.value.None != undefined) value = element.value.None;

          data.push({
            key: element.key,
            value: value,
          });
        });

        resolve(data);
      });

      emit(`GET_STATE`);
    });
  },

  get: async (method: string, args?: any): Promise<any> => {
    return new Promise((resolve, _reject) => {
      listen(`RECEIVE_${method}`, (event) => {
        resolve(event.payload);
      });

      emit(`GET_${method}`);
    });
  },

  set: async (method: string, args?: any): Promise<any> => {
    return new Promise((resolve, _reject) => {
      listen(`RECEIVE_${method}`, (event) => {
        resolve(event.payload);
      });

      emit(`SET_${method}`, args);
    });
  },

  get_nodes: async (): Promise<any> => {
    return new Promise((resolve, _reject) => {
      listen(`RECEIVE_NODES`, (event) => {
        let data = JSON.parse(event.payload as string) as BackendNode[];
        resolve(data);
      });

      emit(`GET_NODES`);
    });
  },
};

export default Interface;
