import { useCallback, useState } from 'react';
import axios from 'axios';
import toWav from 'audiobuffer-to-wav';

const useFileUpload = (
  file: File | null,
  token: string | null,
  audioId: string | null,
  setError: (error: string | null) => void
) => {
  const [isUploading, setIsUploading] = useState(false);

  const uploadActualFile = useCallback(async () => {
    if (isUploading) {
      setError('Already uploading');
      return false;
    }
    setIsUploading(true);

    if (!file || !token || !audioId) {
      setError('Missing file, token or audioId');
      setIsUploading(false);
      return false;
    }

    console.log('Converting file to wav');
    const formData = new FormData();
    let arrayBuffer = await file.arrayBuffer();
    let audioBuffer = await new AudioContext().decodeAudioData(arrayBuffer);
    let wav = toWav(audioBuffer);
    console.log('Converted file to wav');

    console.log('Creating new file');
    let blob = new Blob([new Uint8Array(wav)], { type: 'audio/wav' });
    let file_name = file.name.split('.')[0];
    let new_file_name = file_name + '.wav';
    console.log('Filename: ' + new_file_name);
    let wavefile = new File([blob], new_file_name, { type: 'audio/wav' });
    formData.append('file_data', wavefile);
    console.log('Created new file');


    try {
      console.log('Uploading file');
      let response = await axios.post(`https://api.drumni.com/api/file/${audioId}`, formData, {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'multipart/form-data',
        },
      });
      switch (response.status) {
        case 200:
          setError(null);
          setIsUploading(false);
          return true;
        default:
          setIsUploading(false);
          setError('Error on upload: ' + response.statusText);
          return false;
      }

    } catch (error: any) {
      setError('Error on upload: ' + error.message);
      setIsUploading(false);
      return false;
    }
  }, [file, token, audioId, setError]);

  return uploadActualFile;
};

export default useFileUpload;
