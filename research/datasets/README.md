# Malware Datasets

**For Phase 2 (Months 9-16)**

This directory will contain malware behavior datasets for training the AI model.

## Recommended Datasets

### 1. EMBER
- **Link**: https://github.com/elastic/ember
- **Size**: ~1M samples
- **Format**: PE files + features
- **Use**: Training behavioral classifier

### 2. SOREL-20M
- **Link**: https://github.com/sophos/SOREL-20M
- **Size**: 20M samples
- **Format**: Labeled malware samples
- **Use**: Large-scale training

### 3. VirusShare
- **Link**: https://virusshare.com/
- **Size**: Varies
- **Format**: Raw malware samples
- **Use**: Additional training data

## Directory Structure (Future)

```
datasets/
├── ember/
│   ├── train_features.json
│   └── train_labels.json
├── sorel/
│   └── samples/
└── processed/
    ├── behavior_sequences/
    └── system_calls/
```

## Data Processing Pipeline (Phase 2)

1. **Download** raw datasets
2. **Extract** behavior features (system calls, API calls)
3. **Label** with threat categories
4. **Split** into train/validation/test
5. **Export** to format suitable for PyTorch training

## Legal & Ethical Considerations

**IMPORTANT**:
- Only use datasets from legitimate security research sources
- Never distribute malware samples
- Analyze in isolated VMs only
- Follow responsible disclosure practices

## Storage

Datasets are large (10GB+). Consider:
- External SSD for storage
- Git LFS for version control (or don't commit at all)
- Cloud storage (encrypted)

---

**Phase 1**: Don't worry about this yet. Focus on the kernel driver.
**Phase 2**: Return here when ready to train AI models.
