import { Box, Fade, ImageList, ImageListItem, Paper, Popper, Skeleton, TextField } from '@mui/material'
import React, { useEffect, useState } from 'react';
import SearchIcon from '@mui/icons-material/Search';
import { useTranslation } from 'react-i18next';
import { debounce } from 'lodash';
import { useSelector } from 'react-redux';
import { RootState } from '../store/store';
import { invoke } from '@tauri-apps/api';
import { use } from 'i18next';

interface MediaElement {
  webm: {
    size: number;
    duration: number;
    preview: string;
    url: string;
    dims: [number, number];
  };
  tinywebm: {
    preview: string;
    dims: [number, number];
    url: string;
    size: number;
    duration: number;
  };
  webp_transparent: {
    url: string;
    size: number;
    preview: string;
    dims: [number, number];
    duration: number;
  };
  tinygif: {
    size: number;
    preview: string;
    dims: [number, number];
    duration: number;
    url: string;
  };
  tinymp4: {
    dims: [number, number];
    preview: string;
    size: number;
    url: string;
    duration: number;
  };
  gif: {
    size: number;
    preview: string;
    duration: number;
    dims: [number, number];
    url: string;
  };
  nanowebp_transparent: {
    duration: number;
    url: string;
    preview: string;
    size: number;
    dims: [number, number];
  };
  tinywebp_transparent: {
    preview: string;
    size: number;
    dims: [number, number];
    url: string;
    duration: number;
  };
  nanomp4: {
    size: number;
    duration: number;
    dims: [number, number];
    preview: string;
    url: string;
  };
  nanowebm: {
    url: string;
    dims: [number, number];
    preview: string;
    size: number;
    duration: number;
  };
  mp4: {
    duration: number;
    preview: string;
    url: string;
    size: number;
    dims: [number, number];
  };
  loopedmp4: {
    url: string;
    size: number;
    preview: string;
    dims: [number, number];
    duration: number;
  };
  nanogif: {
    duration: number;
    size: number;
    preview: string;
    dims: [number, number];
    url: string;
  };
  mediumgif: {
    url: string;
    dims: [number, number];
    size: number;
    preview: string;
    duration: number;
  };
};

export interface GifResult {
  id: string;
  title: string;
  content_description: string;
  content_rating: string;
  h1_title: string;
  media: MediaElement[];
  bg_color: string;
  created: number;
  itemurl: string;
  url: string;
  tags: string[];
  flags: string[];
  shares: number;
  hasaudio: boolean;
  hascaption: boolean;
  source_id: string;
  composite: any;
}

interface GidResultContainer {
  results: GifResult[];
  next: string;
}

interface EmptyResult {

}

interface ErrorElement {
  code: number;
  error: string;
}

interface GifSearchProps {
  open: boolean
  anchor: HTMLElement | undefined
  onGifSelected?: (gif: GifResult) => void
}

const handleSearchChange = debounce(async (
  value: string,
  tenorApiKey: string | undefined,
  setItemData: React.Dispatch<React.SetStateAction<GidResultContainer | ErrorElement | EmptyResult[]>>
) => {
  if (tenorApiKey) {
    if (value.length === 0) {
      const result = await invoke('get_tenor_trending_results', {
        apiKey: tenorApiKey
      });
      const resultObj = JSON.parse(result as string);
      setItemData(resultObj as GidResultContainer);
      return;
    }
    const result = await invoke('get_tenor_search_results', {
      apiKey: tenorApiKey,
      query: value,
      limit: 10,
      pos: 0,
    });
    const resultObj = JSON.parse(result as string);
    console.log(resultObj);

    setItemData(resultObj as GidResultContainer);
  }
}, 1000);

function GifSearch(props: Readonly<GifSearchProps>) {
  const frontendSettings = useSelector((state: RootState) => state.reducer.frontendSettings);
  const tenorApiKey = frontendSettings.api_keys?.tenor;

  const { t } = useTranslation();
  const [search, setSearch] = useState("");
  const [itemData, setItemData] = useState<GidResultContainer | ErrorElement | EmptyResult[]>([{}, {}, {}, {}, {}, {}]);

  useEffect(() => {
    handleSearchChange(search, tenorApiKey, setItemData);
  }, [search]);

  useEffect(() => {
    if(!tenorApiKey || tenorApiKey.length === 0) {
      return;
    }

    invoke('get_tenor_trending_results', {
      apiKey: tenorApiKey
    }).then((result) => {
      const resultObj = JSON.parse(result as string);
      if(resultObj.error) {
        console.log(resultObj.error);
        return;
      }
      setItemData(resultObj as GidResultContainer);
    }).catch(e => {
      console.log(e);
    });
  }, [tenorApiKey]);

  function handleGifClick(gif: GifResult) {
    console.log("clicked");
    if (props.onGifSelected) {
      props.onGifSelected(gif);
    }
  }

  console.log("Data: ", itemData);
  return (
    <Popper open={props.open} anchorEl={props.anchor} transition>
      {({ TransitionProps }) => (
        <Fade {...TransitionProps}>
          <Paper sx={{ p: 1 }}>
            <Box>
              <TextField
                fullWidth
                label={t("Search Tenor")}
                InputProps={{
                  endAdornment: <SearchIcon />,
                }}
                size='small'
                value={search}
                onChange={e => {
                  setSearch(e.target.value);
                }}
              />
            </Box>
            <Box sx={{ width: '100%' }}>
              <ImageList sx={{ width: 500, height: 450 }} cols={3} rowHeight={164} >
                {((itemData as GidResultContainer)?.results ?? itemData).map((item, i) => {
                  if (Object.keys(item).length === 0) {
                    return (
                      <ImageListItem key={i}>
                        <Skeleton variant="rectangular" width={180} height={100} />
                      </ImageListItem>
                    );
                  } else {
                    const imgElement = item as GifResult;

                    return (
                      <ImageListItem key={imgElement.id} sx={{ cursor: 'pointer' }}>
                        <img src={imgElement.media[0].nanogif.url} alt={imgElement.title} loading="lazy" onClick={() => handleGifClick(imgElement)} />
                      </ImageListItem>
                    );
                  }
                })}
              </ImageList>
            </Box>
            {tenorApiKey && tenorApiKey.length > 0 ? null : <Box>{t("Tenor API Key not set")}</Box>}
          </Paper>
        </Fade>
      )}
    </Popper>
  )
}

export default GifSearch