import { useCallback } from 'react';
import axios from 'axios';

const useFileInformation = (
  file: File | null,
  token: string | null,
  user: string | null,
  setError: (error: string | null) => void,
  setAudioId: (audioId: string | null) => void
) => {
  const validateFile = useCallback(() => {
    if (!file) {
      setError('No file selected');
      return false;
    }

    if (file.size > 100000000) {
      setError('File is too large');
      return false;
    }

    if (!file.type.includes('audio')) {
      setError('File is not an audio file');
      return false;
    }

    setError(null);
    return true;
  }, [file, setError]);


  const uploadFileInformation = useCallback(async () => {
    if (!file || !token) {
      setError('Missing file or token');
      return false;
    }

    if (!validateFile()) {
      return false;
    }

    // convert file to wav

    let file_name = file.name.split('.')[0];
    let new_file_name = file_name + '.wav';

    let new_file_type = 'audio/wav';

    const fileInfo = {
      name: new_file_name || '',
      size: file.size || 0,
      mime_type: new_file_type || '',
      owner: user || '',
    };

    console.log('Uploading file information');
    console.log(fileInfo);

    try {
      const response = await axios.post(
        'https://api.drumni.com/api/upload',
        fileInfo,
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      const data = response.data;
      if (data) {
        if (data === 'Error') {
          setError('Error: on api upload');
          return false;
        }
        setAudioId(data);
        setError(null);
        return true;
      } else {
        setError('No data returned from server');
        return false;
      }
    } catch (error) {
      setError("Error Uploading File Information")
      return false;
    }
  }, [file, token, setError, setAudioId]);

  return uploadFileInformation;
};



export default useFileInformation;
