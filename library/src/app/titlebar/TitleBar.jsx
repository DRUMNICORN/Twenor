import { appWindow } from "@tauri-apps/api/window";
import Button from "./components/Button";
import { useState } from "react";
import "../../styles/TitleBar.scss";
import Api from "../Api";

function TitleBar() {
  const [isReloading, setReloading] = useState(false);

  return (
    <>
      <div className="titlebar-right" data-tauri-drag-region>
        <Button
          name="import"
          link="https://api.iconify.design/ant-design/import-outlined.svg"
          onClick={() => requestImportXml()}
        />
        <Button
          name="export"
          link="https://api.iconify.design/ant-design/export-outlined.svg"
          onClick={() => requestExportXml()}
        />
      </div>
      <div className="titlebar" data-tauri-drag-region>
        <Button
          name="reload"
          link="https://api.iconify.design/ant-design/reload-outlined.svg"
          onClick={() => requestReload()}
        />

        <Button
          name="config"
          link="https://api.iconify.design/ant-design/setting-outlined.svg"
          onClick={() => Api.toggleConfig()}
        />

        <Button
          name="minimize"
          link="https://api.iconify.design/codicon/chrome-minimize.svg"
          onClick={() => appWindow.minimize()}
        />

        <Button
          name="maximize"
          link="https://api.iconify.design/codicon/chrome-maximize.svg"
          onClick={() => appWindow.toggleMaximize()}
        />

        <Button
          name="close"
          link="https://api.iconify.design/ant-design/close-outlined.svg"
          onClick={() => appWindow.close()}
        />
      </div>
    </>
  );

  async function requestImportXml() {
    // open file dialog and select xml file to import into the track library

    let xml_path = await Api.requestOpenFileDialog();
    console.log(` <= Import Xml: ${xml_path}`);
    Api.requestImportXml(xml_path);
  }

  function requestExportXml() {}

  function requestReload() {
    if (isReloading) return;
    Api.requestReload();
    let you_see = document.getElementById("reload").childNodes[0];
    you_see.classList.add("titlebar-reload-animate");
    // api.libraryReload();
    setReloading(true);
    setTimeout(() => {
      you_see.classList.remove("titlebar-reload-animate");
      setReloading(false);
    }, 2000);
  }
}

export default TitleBar;
