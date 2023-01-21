/**
 * @file Interface.tsx
 * @description Interface for connection to backend
 *
 */

// it uses tauri emit and listen to communicate with backend
import { useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";

const Interface = {
  openFileDialog: async (title: string, filters: any): Promise<any> => {
    return new Promise((resolve, reject) => {
      listen(`RECEIVE_OPEN_FILE_DIALOG`, (event) => {
        resolve(event.payload);
      });

      let data = {
        FileDialog: {
          title: title,
          filters: filters,
        },
      };

      emit(`OPEN_FILE_DIALOG`, JSON.stringify(data));
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
  // set: async (method: string, args?: any): Promise<any> => {
  //   return new Promise((resolve, _reject) => {
  //     listen(`RECEIVE_${method}`, (event) => {
  //       resolve(event.payload);
  //     });
  //     // if (!args) args = {};
  //     emit(
  //       `SET_${method}`,
  //       // JSON.stringify({
  //       //   Data: data,
  //       // })
  //     );
  //   });
  // },
};

export default Interface;
