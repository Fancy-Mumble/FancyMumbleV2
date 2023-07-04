import { Box, Button, Card, CardActionArea, CardActions, CardContent, CardMedia, Link, Typography } from "@mui/material";
import "./styles/UserInfo.css";
import "./styles/common.css"
import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { s } from "@tauri-apps/api/app-373d24a3";

interface UrlPreviewProps {
    href: string;
}


interface UrlPreviewData {
    title: string,
    description: string,
    image: string
}

function UrlPreview(props: UrlPreviewProps) {

    let [urlPreviewData, setUrlPreviewData] = useState<UrlPreviewData | undefined>(undefined);

    useEffect(() => {
        invoke<string>('get_open_graph_data_from_website', { url: props.href }).then((data) => {
            setUrlPreviewData(JSON.parse(data));
        });
    }, []);

    const createOpenGraphData = useMemo(() => {
        console.log(urlPreviewData);
        if (urlPreviewData) {
            return (
                <Card sx={{ maxWidth: 345 }}>
                    <CardActionArea onClick={() => window.open(props.href, '_blank')}>
                        <CardMedia
                            component="img"
                            height="140"
                            image={urlPreviewData.image}
                            alt={urlPreviewData.title}
                        />
                        <CardContent>
                            <Typography gutterBottom variant="h5" component="div">
                                {urlPreviewData.title}
                            </Typography>
                            <Typography variant="body2" color="text.secondary">
                                {urlPreviewData.description}
                            </Typography>
                        </CardContent>
                    </CardActionArea>
                    <CardActions>
                        <Button size="small" color="primary" href={props.href} target="_blank" sx={{fontSize: 9, textTransform: 'lowercase'}}>
                            {props.href}
                        </Button>
                    </CardActions>
                </Card>
            )
        } else {
            return (
                <Box>
                    <Link href={props.href}>{props.href}</Link>
                </Box>
            );
        }
    }, [urlPreviewData]);

    return (
        <Box>
            {createOpenGraphData}
        </Box>
    );
}

export default UrlPreview;