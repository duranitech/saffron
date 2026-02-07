#!/usr/bin/env python3
"""
SID Validator - Validates all ingredient JSON files against the SID schema.
Usage: python sid/tools/validate.py
"""

import json
import sys
import os
from pathlib import Path

def validate_ingredient(data, schema_path=None):
    """Basic validation without jsonschema dependency."""
    errors = []
    
    # Required fields
    required = ["id", "name", "category", "composition", "physical"]
    for field in required:
        if field not in data:
            errors.append(f"Missing required field: {field}")
    
    # ID format
    if "id" in data:
        import re
        if not re.match(r'^[a-z][a-z0-9_]*$', data["id"]):
            errors.append(f"Invalid ID format: {data['id']} (must match ^[a-z][a-z0-9_]*$)")
    
    # Name must have English
    if "name" in data:
        if "en" not in data["name"]:
            errors.append("name.en is required")
    
    # Category validation
    valid_categories = ["protein", "fat", "carbohydrate", "liquid", "seasoning", "produce", "dairy"]
    if "category" in data and data["category"] not in valid_categories:
        errors.append(f"Invalid category: {data['category']}. Must be one of: {valid_categories}")
    
    # Composition validation
    if "composition" in data:
        comp = data["composition"]
        pct_fields = ["water", "protein", "total_fat", "carbohydrates"]
        for field in pct_fields:
            if field in comp:
                if not (0 <= comp[field] <= 100):
                    errors.append(f"composition.{field} must be 0-100, got {comp[field]}")
        
        if "ph" in comp:
            if not (0 <= comp["ph"] <= 14):
                errors.append(f"composition.ph must be 0-14, got {comp['ph']}")
    
    # Sources should be cited
    if "sources" not in data or len(data.get("sources", [])) == 0:
        errors.append("Warning: No sources cited")
    
    return errors


def main():
    base_dir = Path(__file__).parent.parent
    data_dir = base_dir / "data"
    
    if not data_dir.exists():
        print(f"Data directory not found: {data_dir}")
        sys.exit(1)
    
    total = 0
    passed = 0
    failed = 0
    warnings = 0
    
    for json_file in sorted(data_dir.rglob("*.json")):
        total += 1
        rel_path = json_file.relative_to(base_dir)
        
        try:
            with open(json_file) as f:
                data = json.load(f)
        except json.JSONDecodeError as e:
            print(f"  FAIL  {rel_path}: Invalid JSON - {e}")
            failed += 1
            continue
        
        errors = validate_ingredient(data)
        real_errors = [e for e in errors if not e.startswith("Warning")]
        warns = [e for e in errors if e.startswith("Warning")]
        
        if real_errors:
            print(f"  FAIL  {rel_path}:")
            for err in real_errors:
                print(f"         - {err}")
            failed += 1
        else:
            status = "PASS" if not warns else "WARN"
            print(f"  {status}  {rel_path} ({data.get('name', {}).get('en', 'unknown')})")
            if warns:
                for w in warns:
                    print(f"         - {w}")
                warnings += 1
            passed += 1
    
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {warnings} warnings out of {total} files")
    
    if failed > 0:
        sys.exit(1)
    
    print("All validations passed!")


if __name__ == "__main__":
    main()
