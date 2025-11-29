# AI Models

**For Phase 2+ (Months 9-16)**

This directory will contain trained AI models for threat detection.

## Model Architecture (Planned)

### Transformer-based SLM (Small Language Model)
- **Input**: Sequence of system calls/API calls
- **Output**: Threat probability (0-100%)
- **Size**: <400MB (INT8 quantized)
- **Latency**: <50ms (RTX), <100ms (NPU)

## Model Lifecycle

```
1. Training (Python/PyTorch)
   ├── Define architecture
   ├── Train on malware datasets
   ├── Validate accuracy
   └── Export to ONNX

2. Quantization
   ├── INT8 conversion (reduce size)
   └── Verify accuracy retention

3. Encryption (Phase 4)
   ├── AES-256 encryption
   └── Obfuscated decryption

4. Deployment
   ├── Load in CuraCore service
   └── Inference via ONNX Runtime
```

## Directory Structure (Future)

```
models/
├── raw/
│   └── threat_classifier.pt       # PyTorch model
├── onnx/
│   └── threat_classifier.onnx     # ONNX export
├── quantized/
│   └── threat_classifier_int8.onnx
└── encrypted/
    └── threat_classifier.enc       # Production model
```

## Model Training (Phase 2)

### Tools
- **PyTorch**: Model development
- **ONNX**: Export format
- **ONNX Runtime**: Inference

### Training Pipeline
```python
# Simplified example
import torch
import torch.nn as nn

class ThreatClassifier(nn.Module):
    def __init__(self):
        super().__init__()
        # Transformer architecture
        self.transformer = nn.Transformer(
            d_model=512,
            nhead=8,
            num_encoder_layers=6
        )
        self.fc = nn.Linear(512, 1)  # Binary: malware or not

    def forward(self, x):
        # x = sequence of system calls
        out = self.transformer(x)
        return torch.sigmoid(self.fc(out))

# Train...
# Export to ONNX...
```

## Model Performance Targets

### Accuracy
- **True Positive Rate**: >95% (detect malware)
- **False Positive Rate**: <1% (don't block legit apps)

### Speed
- **NPU (Eco)**: <100ms per inference
- **RTX (Turbo)**: <50ms per inference

### Size
- **Disk**: ~150MB (quantized)
- **RAM**: <400MB during inference
- **VRAM**: ~500MB (RTX mode)

## Version Control

**Do NOT commit large model files to Git!**

Instead:
- Use Git LFS (Large File Storage)
- Or store separately (cloud storage)
- Only commit model architecture code

## Testing

```python
# Test model accuracy
def test_model(model, test_data):
    tp, fp, tn, fn = 0, 0, 0, 0

    for sample, label in test_data:
        prediction = model.predict(sample)

        if prediction > 0.7 and label == 1:
            tp += 1  # Correctly identified malware
        elif prediction > 0.7 and label == 0:
            fp += 1  # False alarm
        # ...

    accuracy = (tp + tn) / (tp + fp + tn + fn)
    print(f"Accuracy: {accuracy:.2%}")
```

## Security

### Model Protection (Phase 4)
- **Encryption**: AES-256
- **Decryption**: Only in memory (TEE if available)
- **Obfuscation**: Protect decryption keys
- **License**: Tied to hardware ID

### Model Updates
- Cloud delivery (encrypted)
- Version control
- Rollback capability

---

**Phase 1**: Ignore this directory. Focus on kernel driver.
**Phase 2**: Come back here to train your AI model.
**Phase 4**: Implement encryption and protection.
