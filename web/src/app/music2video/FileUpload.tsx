import React, { useRef, useState } from 'react';
import styles from '@/styles/FileUpload.module.scss';

type FileUploadProps = {
  onFileUpload: (file: File) => void;
};

const FileUpload: React.FC<FileUploadProps> = ({ onFileUpload }) => {
  const [dragActive, setDragActive] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleDrag = (e: React.DragEvent<HTMLFormElement>) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave" || e.type === "drop") {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent<HTMLFormElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      onFileUpload(e.dataTransfer.files[0]);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    if (e.target.files && e.target.files[0]) {
      onFileUpload(e.target.files[0]);
    }
  };

  const onButtonClick = () => {
    inputRef.current?.click();
  };

  return (
    <form 
      className={styles.uploadForm} 
      onDragEnter={handleDrag} 
      onDragOver={handleDrag} 
      onDragLeave={handleDrag} 
      onDrop={handleDrop}
    >
      <input 
        ref={inputRef} 
        type="file" 
        className={styles.fileInput} 
        onChange={handleChange} 
      />
      <label 
        htmlFor="file-input" 
        className={dragActive ? styles.dragActive : "" }
      >
        <div>
          <p>Drag and drop your file here or</p>
          <button className={styles.uploadButton} onClick={onButtonClick}>Upload a file</button>
        </div> 
      </label>
    </form>
  );
};

export default FileUpload;
