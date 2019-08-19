package com.tokera.ate.providers;

import com.esotericsoftware.yamlbeans.YamlException;
import com.esotericsoftware.yamlbeans.scalar.ScalarSerializer;
import com.tokera.ate.common.StringTools;
import org.checkerframework.checker.nullness.qual.Nullable;

public class EnumSerializer implements ScalarSerializer<Enum>
{
    private final Class<?> clazz;
     
    public EnumSerializer(Class<?> clazz) {
        this.clazz = clazz;
    }

    @SuppressWarnings("override.return.invalid")
    @Override
    public @Nullable String write(@Nullable Enum object) throws YamlException {
        if (object == null) return "null";
        return object.name().toLowerCase();
    }

    @SuppressWarnings("override.return.invalid")
    @Override
    public @Nullable Enum read(@Nullable String _val) throws YamlException {
        String val = StringTools.makeOneLineOrNull(_val);
        val = StringTools.specialParse(val);
        if (val == null || val.length() <= 0) return null;

        Object[] values = clazz.getEnumConstants();
        if (values == null) return null;

        for (Object thisObj : values) {
            if (thisObj instanceof Enum) {
                Enum thisEnum = (Enum)thisObj;
                if (thisEnum.name().equalsIgnoreCase(val)) {
                    return thisEnum;
                }
            }
        }
        return null;
    }
}
