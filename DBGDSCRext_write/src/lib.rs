#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use DBGDSCRint_read::*;
use Mk_DBGDSCRint_Type::*;
use DBGDSCRint_write::*;
use common::*;
pub fn DBGDSCRext_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        ga_6607: ProductType700c18a878c5601b,
        ga_6613: ProductType700c18a878c5601b,
        ga_6601: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #104648u : u32
        let s_0_2: u32 = 104648;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #104648u : u32
        let s_0_4: u32 = 104648;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call DBGDSCRint_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_6);
        // D s_0_8: write-var ga#6601 <= s_0_7
        fn_state.ga_6601 = s_0_7;
        // D s_0_9: read-var ga#6601.0:struct
        let s_0_9: u32 = fn_state.ga_6601._0;
        // D s_0_10: read-var r.0:struct
        let s_0_10: u32 = fn_state.r._0;
        // C s_0_11: const #15s : i
        let s_0_11: i128 = 15;
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // D s_0_13: cast zx s_0_10 -> bv
        let s_0_13: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_14: bit-extract s_0_13 s_0_11 s_0_12
        let s_0_14: Bits = (Bits::new(
            ((s_0_13) >> (s_0_11)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: bool = ((s_0_14.value()) != 0);
        // C s_0_16: const #1s : i
        let s_0_16: i128 = 1;
        // C s_0_17: const #15s : i
        let s_0_17: i128 = 15;
        // D s_0_18: cast zx s_0_9 -> bv
        let s_0_18: Bits = Bits::new(s_0_9 as u128, 32u16);
        // D s_0_19: cast zx s_0_15 -> bv
        let s_0_19: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_20: const #1u : u64
        let s_0_20: u64 = 1;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 64u16);
        // C s_0_22: lsl s_0_21 s_0_16
        let s_0_22: Bits = s_0_21 << s_0_16;
        // C s_0_23: sub s_0_22 s_0_21
        let s_0_23: Bits = ((s_0_22) - (s_0_21));
        // D s_0_24: and s_0_19 s_0_23
        let s_0_24: Bits = ((s_0_19) & (s_0_23));
        // D s_0_25: lsl s_0_24 s_0_17
        let s_0_25: Bits = s_0_24 << s_0_17;
        // C s_0_26: lsl s_0_23 s_0_17
        let s_0_26: Bits = s_0_23 << s_0_17;
        // C s_0_27: cmpl s_0_26
        let s_0_27: Bits = !s_0_26;
        // D s_0_28: and s_0_18 s_0_27
        let s_0_28: Bits = ((s_0_18) & (s_0_27));
        // D s_0_29: or s_0_28 s_0_25
        let s_0_29: Bits = ((s_0_28) | (s_0_25));
        // D s_0_30: cast reint s_0_29 -> u32
        let s_0_30: u32 = (s_0_29.value() as u32);
        // D s_0_31: call Mk_DBGDSCRint_Type(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = Mk_DBGDSCRint_Type(
            state,
            tracer,
            s_0_30,
        );
        // D s_0_32: call DBGDSCRint_write(s_0_31)
        let s_0_32: () = DBGDSCRint_write(state, tracer, s_0_31);
        // C s_0_33: const #() : ()
        let s_0_33: () = ();
        // S s_0_34: call DBGDSCRint_read(s_0_33)
        let s_0_34: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_33);
        // D s_0_35: write-var ga#6607 <= s_0_34
        fn_state.ga_6607 = s_0_34;
        // D s_0_36: read-var ga#6607.0:struct
        let s_0_36: u32 = fn_state.ga_6607._0;
        // D s_0_37: read-var r.0:struct
        let s_0_37: u32 = fn_state.r._0;
        // C s_0_38: const #12s : i
        let s_0_38: i128 = 12;
        // C s_0_39: const #1s : i
        let s_0_39: i128 = 1;
        // D s_0_40: cast zx s_0_37 -> bv
        let s_0_40: Bits = Bits::new(s_0_37 as u128, 32u16);
        // D s_0_41: bit-extract s_0_40 s_0_38 s_0_39
        let s_0_41: Bits = (Bits::new(
            ((s_0_40) >> (s_0_38)).value(),
            u16::try_from(s_0_39).unwrap(),
        ));
        // D s_0_42: cast reint s_0_41 -> u8
        let s_0_42: bool = ((s_0_41.value()) != 0);
        // C s_0_43: const #1s : i
        let s_0_43: i128 = 1;
        // C s_0_44: const #12s : i
        let s_0_44: i128 = 12;
        // D s_0_45: cast zx s_0_36 -> bv
        let s_0_45: Bits = Bits::new(s_0_36 as u128, 32u16);
        // D s_0_46: cast zx s_0_42 -> bv
        let s_0_46: Bits = Bits::new(s_0_42 as u128, 1u16);
        // C s_0_47: const #1u : u64
        let s_0_47: u64 = 1;
        // C s_0_48: cast zx s_0_47 -> bv
        let s_0_48: Bits = Bits::new(s_0_47 as u128, 64u16);
        // C s_0_49: lsl s_0_48 s_0_43
        let s_0_49: Bits = s_0_48 << s_0_43;
        // C s_0_50: sub s_0_49 s_0_48
        let s_0_50: Bits = ((s_0_49) - (s_0_48));
        // D s_0_51: and s_0_46 s_0_50
        let s_0_51: Bits = ((s_0_46) & (s_0_50));
        // D s_0_52: lsl s_0_51 s_0_44
        let s_0_52: Bits = s_0_51 << s_0_44;
        // C s_0_53: lsl s_0_50 s_0_44
        let s_0_53: Bits = s_0_50 << s_0_44;
        // C s_0_54: cmpl s_0_53
        let s_0_54: Bits = !s_0_53;
        // D s_0_55: and s_0_45 s_0_54
        let s_0_55: Bits = ((s_0_45) & (s_0_54));
        // D s_0_56: or s_0_55 s_0_52
        let s_0_56: Bits = ((s_0_55) | (s_0_52));
        // D s_0_57: cast reint s_0_56 -> u32
        let s_0_57: u32 = (s_0_56.value() as u32);
        // D s_0_58: call Mk_DBGDSCRint_Type(s_0_57)
        let s_0_58: ProductType700c18a878c5601b = Mk_DBGDSCRint_Type(
            state,
            tracer,
            s_0_57,
        );
        // D s_0_59: call DBGDSCRint_write(s_0_58)
        let s_0_59: () = DBGDSCRint_write(state, tracer, s_0_58);
        // C s_0_60: const #() : ()
        let s_0_60: () = ();
        // S s_0_61: call DBGDSCRint_read(s_0_60)
        let s_0_61: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_60);
        // D s_0_62: write-var ga#6613 <= s_0_61
        fn_state.ga_6613 = s_0_61;
        // D s_0_63: read-var ga#6613.0:struct
        let s_0_63: u32 = fn_state.ga_6613._0;
        // D s_0_64: read-var r.0:struct
        let s_0_64: u32 = fn_state.r._0;
        // C s_0_65: const #2s : i
        let s_0_65: i128 = 2;
        // C s_0_66: const #4s : i
        let s_0_66: i128 = 4;
        // D s_0_67: cast zx s_0_64 -> bv
        let s_0_67: Bits = Bits::new(s_0_64 as u128, 32u16);
        // D s_0_68: bit-extract s_0_67 s_0_65 s_0_66
        let s_0_68: Bits = (Bits::new(
            ((s_0_67) >> (s_0_65)).value(),
            u16::try_from(s_0_66).unwrap(),
        ));
        // D s_0_69: cast reint s_0_68 -> u8
        let s_0_69: u8 = (s_0_68.value() as u8);
        // C s_0_70: const #4s : i
        let s_0_70: i128 = 4;
        // C s_0_71: const #2s : i
        let s_0_71: i128 = 2;
        // D s_0_72: cast zx s_0_63 -> bv
        let s_0_72: Bits = Bits::new(s_0_63 as u128, 32u16);
        // D s_0_73: cast zx s_0_69 -> bv
        let s_0_73: Bits = Bits::new(s_0_69 as u128, 4u16);
        // C s_0_74: const #1u : u64
        let s_0_74: u64 = 1;
        // C s_0_75: cast zx s_0_74 -> bv
        let s_0_75: Bits = Bits::new(s_0_74 as u128, 64u16);
        // C s_0_76: lsl s_0_75 s_0_70
        let s_0_76: Bits = s_0_75 << s_0_70;
        // C s_0_77: sub s_0_76 s_0_75
        let s_0_77: Bits = ((s_0_76) - (s_0_75));
        // D s_0_78: and s_0_73 s_0_77
        let s_0_78: Bits = ((s_0_73) & (s_0_77));
        // D s_0_79: lsl s_0_78 s_0_71
        let s_0_79: Bits = s_0_78 << s_0_71;
        // C s_0_80: lsl s_0_77 s_0_71
        let s_0_80: Bits = s_0_77 << s_0_71;
        // C s_0_81: cmpl s_0_80
        let s_0_81: Bits = !s_0_80;
        // D s_0_82: and s_0_72 s_0_81
        let s_0_82: Bits = ((s_0_72) & (s_0_81));
        // D s_0_83: or s_0_82 s_0_79
        let s_0_83: Bits = ((s_0_82) | (s_0_79));
        // D s_0_84: cast reint s_0_83 -> u32
        let s_0_84: u32 = (s_0_83.value() as u32);
        // D s_0_85: call Mk_DBGDSCRint_Type(s_0_84)
        let s_0_85: ProductType700c18a878c5601b = Mk_DBGDSCRint_Type(
            state,
            tracer,
            s_0_84,
        );
        // D s_0_86: call DBGDSCRint_write(s_0_85)
        let s_0_86: () = DBGDSCRint_write(state, tracer, s_0_85);
        // C s_0_87: const #17648u : u32
        let s_0_87: u32 = 17648;
        // N s_0_88: write-reg s_0_87 <= s_0_0
        let s_0_88: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_87 as isize, s_0_0);
            tracer.write_register(s_0_87 as isize, s_0_0);
        };
        // N s_0_89: return
        return;
    }
}
