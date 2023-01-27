/**
 * @file Datatypes.tsx
 * @description Datatypes for the frontend
 *
 */

export type Property = {
  key: string;
  value: {
    Object?: string;
    I32?: number;
    F32?: number;
    Bool?: boolean;
    String?: string;
    None?: null;
  };
};

export type BackendNode = {
  id: number;
  name: string;
  type: string;
  children: Array<BackendNode>;
};
