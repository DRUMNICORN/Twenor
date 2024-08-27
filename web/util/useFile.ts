import { useEffect, useState } from 'react';
import useFileInformation from './useFileInformation';
import useFileUpload from './useFileUpload';

import axios from 'axios';


interface UseFileHookResult {
  isLoading: boolean;
  audioId: string | null;
  error: string | null;
  downloadFile: (audioId: string) => void;
  uploadFile: (file: File | null) => void;
  file: File | null;
}

const useFile = (token: string | null, user: string | null): UseFileHookResult => {
  const [file, setFile] = useState<File | null>(null);
  const [audioId, setAudioId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isDownloaded, setIsDownloaded] = useState(false);

  const uploadFileInformation = useFileInformation(file, token, user, setError, setAudioId);
  const uploadActualFile = useFileUpload(file, token, audioId, setError);

  useEffect(() => {
    const uploadAndFetchMetadata = async () => {
      if (isLoading) {
        setError('Already uploading');
        return;
      }
      if (isDownloaded) {
        setError('Already downloaded');
        return;
      }

      setIsLoading(true);
      const fileInfoUploaded = await uploadFileInformation();
      if (fileInfoUploaded) {
        await uploadActualFile();
      } else {
        setError('Error uploading file information');
      }
      setIsLoading(false);
    };

    if (!file) {
      setError('No file selected');
      return;
    } else if (!token) {
      setError('No token provided');
    } else if (file && token) {
      uploadAndFetchMetadata();
    }
  }, [file, token, uploadFileInformation, uploadActualFile]);


  const getFilename = async (audioIdFromFile: string) => {
    if (!audioIdFromFile) {
      setError('No audio id provided');
      return;
    } else if (!token) {
      setError('No token provided');
      return;
    } else if (audioIdFromFile === audioId) {
      return;
    }

    console.log('getting filename');
    setAudioId(audioIdFromFile);
    setIsLoading(true);
    try {
      console.log('getting filename');
      const response = await axios.get(`https://api.drumni.com/api/file/${audioIdFromFile}`, {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
      });
      console.log('filename retrieved');
      const filename = response.data;
      console.log(`filename: ${filename}`);
      return filename;
    } catch (error) {
      console.error(error);
      setError("Couldn't get filename");
    }
    console.log('done getting filename');
    setIsLoading(false);
  };

  const downloadFile = async (audioIdDownload: string) => {
    if (!audioIdDownload) {
      setError('No audio id provided');
      return;
    } else if (!token) {
      setError('No token provided');
      return;
    } else if (audioIdDownload === audioId) {
      return;
    }

    console.log('downloading file');
    setIsLoading(true);
    try {
      const response = await axios.get(`https://api.drumni.com/api/download/${audioIdDownload}`, {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        responseType: 'blob',
      });

      const filename = await getFilename(audioIdDownload);
      if (!filename) {
        setError("Couldn't get filename");
        return;
      }
      let extension = filename.split('.').pop();
      let type = '';
      console.log(`extension: ${extension}`);
      switch (extension) {
        case 'mp3':
          type = 'audio/mpeg';
          break;
        case 'wav':
          type = 'audio/mpeg';
          break;
        case 'ogg':
          type = 'audio/mpeg';
          break;
        case 'flac':
          type = 'audio/mpeg';
          break;
        default:
          type = 'audio/mpeg';
          break;
      }

      const blob = new Blob([response.data], { type: type });
      const file = new File([blob], filename, { type: type });

      setAudioId(audioIdDownload);
      setFile(file);
      setIsDownloaded(true);
    } catch (error) {
      console.error(error);
      setError("Couldn't download file");
    }
    console.log('done downloading file');
    setIsLoading(false);
  };

  const reset = () => {
    setFile(null);
    setAudioId(null);
    setAudioId(null);
    setError(null);
    setIsLoading(false);
  };

  const uploadFile = async (file: File | null) => {
    if (!file) {
      reset();
      return;
    }
    setFile(file);
    setIsDownloaded(false);
  };

  return { audioId, error, isLoading, downloadFile, uploadFile, file };
};

export default useFile;
