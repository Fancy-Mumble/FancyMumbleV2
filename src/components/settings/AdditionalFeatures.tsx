import { Alert, Box, Container, Divider, Grid, Switch, TextField, Typography } from '@mui/material';
import { G } from '@tauri-apps/api/path-e12e0e34';
import React, { useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../../store/store';
import { LinkPreviewSettings, updateLinkPreview } from '../../store/features/users/frontendSettings';
import { invoke } from '@tauri-apps/api';


interface AdditionalFeaturesProps {
}

const AdditionalFeatures: React.FC<AdditionalFeaturesProps> = React.memo(({ }) => {
    const linkPreview = useSelector((state: RootState) => state.reducer.frontendSettings).linkPreview;
    const dispatch = useDispatch();

    let [errorMessage, setErrorMessage] = useState('');
    const label = { inputProps: { 'aria-label': 'Link Preview' } };

    function showErrorMessage() {
        if (errorMessage) {
            return <Alert severity="error">{errorMessage}</Alert>
        }
    }

    function updateEnabled() {
        updateLinkPreviewState((state) => {
            let newState = !linkPreview.enabled;
            state.enabled = newState;

            return state;
        });
    }

    function updateAllowAll() {
        updateLinkPreviewState((state) => {
            let newState = !linkPreview.allow_all;
            state.allow_all = newState;

            return state;
        });
    }

    function updateAllowedUrls(urls: string[]) {
        updateLinkPreviewState((state) => {
            state.urls = urls;
            console.log(state);
            return state;
        });
    }

    function updateLinkPreviewState(stateChangeFunction: (newState: LinkPreviewSettings) => LinkPreviewSettings) {
        let stateClone = JSON.parse(JSON.stringify(linkPreview));
        stateClone = stateChangeFunction(stateClone);

        console.log(stateClone);
        invoke('save_frontend_settings', { settingsName: 'link_preview', data: stateClone }).then(() => {
            console.log('Saved settings.');
        }).catch(e => {
            console.log(e);
            setErrorMessage('Failed to save settings.');
        });

        dispatch(updateLinkPreview(stateClone));
    }

    function alloAllSetting() {
        if (!linkPreview.enabled) return (<Box />);

        return (
            <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Typography variant="h6">Allow URLs from all sources</Typography>
                </Grid>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Switch {...label} checked={linkPreview.allow_all || false} onChange={() => updateAllowAll()} />
                </Grid>
            </Grid>
        )
    }

    function linkPreviewSettings() {
        if (!linkPreview.enabled || linkPreview.allow_all) return (<Box />);

        return (
            <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Typography variant="h6">Link Preview Urls</Typography>
                </Grid>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <TextField {...label} value={linkPreview.urls.join('\n')} multiline onChange={(text) => updateAllowedUrls(text.target.value.split(/\r?\n/).map((e) => e.trim()))} maxRows={10} />
                </Grid>
            </Grid>
        )
    }

    return (
        <Box>
            <Container className="settingsContainer">
                {showErrorMessage()}
                <Typography variant="h4">Additional Features</Typography>
                <Divider sx={{ marginBottom: 5 }} />
                <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                    <Grid item xs={4} sm={8} md={6} lg={6}>
                        <Typography variant="h6">Enable Link Preview</Typography>
                    </Grid>
                    <Grid item xs={4} sm={8} md={6} lg={6}>
                        <Switch {...label} checked={linkPreview.enabled} onChange={() => updateEnabled()} />
                    </Grid>
                </Grid>
                {alloAllSetting()}
                {linkPreviewSettings()}
            </Container>
        </Box>
    )
});

export default AdditionalFeatures;