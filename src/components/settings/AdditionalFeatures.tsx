import { Alert, Box, Container, Divider, Grid, Switch, TextField, Typography } from '@mui/material';
import { G } from '@tauri-apps/api/path-e12e0e34';
import React, { useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../../store/store';
import { LinkPreviewSettings, updateApiKey, ApiKeys, FrontendSettings, updateFrontendSettings, updateLinkPreview } from '../../store/features/users/frontendSettings';
import { invoke } from '@tauri-apps/api';
import { ActionCreatorWithPayload } from '@reduxjs/toolkit';


interface AdditionalFeaturesProps {
}

const AdditionalFeatures: React.FC<AdditionalFeaturesProps> = React.memo(({ }) => {
    const frontendSettings = useSelector((state: RootState) => state.reducer.frontendSettings);
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
            let newState = !frontendSettings.link_preview.enabled;
            state.link_preview.enabled = newState;

            return state;
        }, updateLinkPreview);
    }

    function updateAllowAll() {
        updateLinkPreviewState((state) => {
            let newState = !frontendSettings.link_preview.allow_all;
            state.link_preview.allow_all = newState;

            return state;
        }, updateLinkPreview);
    }

    function updateAllowedUrls(urls: string[]) {
        updateLinkPreviewState((state) => {
            state.link_preview.urls = urls;
            return state;
        }, updateLinkPreview);
    }

    function updateTenorApiKey(value: string): void {
        updateLinkPreviewState((state) => {
            state.api_keys.tenor = value;
            return state;
        }, updateApiKey);
    }

    function updateLinkPreviewState(stateChangeFunction: (newState: FrontendSettings) => FrontendSettings, call: ActionCreatorWithPayload<any, any>) {
        let stateClone = JSON.parse(JSON.stringify(frontendSettings));
        stateClone = stateChangeFunction(stateClone);

        let storedData = stateClone.link_preview;
        console.log(storedData);
        invoke('save_frontend_settings', { settingsName: 'general', data: { 'LinkPreview': storedData } }).then(() => {
            console.log('Saved settings: ', storedData);
        }).catch(e => {
            console.log(e);
            setErrorMessage('Failed to save settings: ' + e);
        });

        dispatch(call(stateClone));
    }

    function allowAllSetting() {
        if (!frontendSettings.link_preview.enabled) return (<Box />);

        return (
            <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Typography variant="h6">Allow URLs from all sources</Typography>
                </Grid>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Switch {...label} checked={frontendSettings.link_preview.allow_all || false} onChange={() => updateAllowAll()} />
                </Grid>
            </Grid>
        )
    }

    function linkPreviewSettings() {
        if (!frontendSettings.link_preview.enabled || frontendSettings.link_preview.allow_all) return (<Box />);

        return (
            <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <Typography variant="h6">Link Preview Urls</Typography>
                </Grid>
                <Grid item xs={4} sm={8} md={6} lg={6}>
                    <TextField {...label} value={frontendSettings.link_preview.urls.join('\n')} multiline onChange={(text) => updateAllowedUrls(text.target.value.split(/\r?\n/))} maxRows={10} />
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
                <Box m={2}>
                    Value: {JSON.stringify(frontendSettings.link_preview)}
                    <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                        <Grid item xs={4} sm={8} md={6} lg={6}>
                            <Typography variant="h6">Enable Link Preview</Typography>
                        </Grid>
                        <Grid item xs={4} sm={8} md={6} lg={6}>
                            <Switch {...label} checked={frontendSettings.link_preview.enabled} onChange={() => updateEnabled()} />
                        </Grid>
                    </Grid>
                    {allowAllSetting()}
                    {linkPreviewSettings()}
                </Box>
                <Divider variant="middle" />
                <Box m={2}>
                    <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                        <Grid item xs={4} sm={8} md={6} lg={6}>
                            <Typography variant="h6">Tenor API Key</Typography>
                        </Grid>
                        <Grid item xs={4} sm={8} md={6} lg={6}>
                            <TextField {...label} value={frontendSettings.api_keys.tenor} onChange={(text) => updateTenorApiKey(text.target.value)} maxRows={10} />
                        </Grid>
                    </Grid>
                </Box>
            </Container>
        </Box>
    )
});

export default AdditionalFeatures;