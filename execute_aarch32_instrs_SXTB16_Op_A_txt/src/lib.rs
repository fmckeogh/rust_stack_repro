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
use R_read::*;
use R_set::*;
use ROR::*;
use common::*;
pub fn execute_aarch32_instrs_SXTB16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    rotation: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        rotation: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        rotation,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var rotation:i64
        let s_0_4: i64 = fn_state.rotation;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call ROR(s_0_3, s_0_5)
        let s_0_6: Bits = ROR(state, tracer, s_0_3, s_0_5);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: read-var d:i64
        let s_0_8: i64 = fn_state.d;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: call R_read(s_0_9)
        let s_0_10: u32 = R_read(state, tracer, s_0_9);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: cast zx s_0_7 -> bv
        let s_0_12: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_13: const #1s : i64
        let s_0_13: i64 = 1;
        // C s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_15: const #7s : i
        let s_0_15: i128 = 7;
        // C s_0_16: add s_0_15 s_0_14
        let s_0_16: i128 = (s_0_15 + s_0_14);
        // D s_0_17: bit-extract s_0_12 s_0_11 s_0_16
        let s_0_17: Bits = (Bits::new(
            ((s_0_12) >> (s_0_11)).value(),
            u16::try_from(s_0_16).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // C s_0_19: const #16s : i
        let s_0_19: i128 = 16;
        // D s_0_20: cast zx s_0_18 -> bv
        let s_0_20: Bits = Bits::new(s_0_18 as u128, 8u16);
        // D s_0_21: bits-cast sx s_0_20 -> bv length s_0_19
        let s_0_21: Bits = s_0_20.sign_extend(s_0_19);
        // D s_0_22: cast reint s_0_21 -> u16
        let s_0_22: u16 = (s_0_21.value() as u16);
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // D s_0_24: cast zx s_0_10 -> bv
        let s_0_24: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_25: cast zx s_0_22 -> bv
        let s_0_25: Bits = Bits::new(s_0_22 as u128, 16u16);
        // C s_0_26: const #15s : i
        let s_0_26: i128 = 15;
        // C s_0_27: const #1u : u64
        let s_0_27: u64 = 1;
        // C s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 64u16);
        // C s_0_29: lsl s_0_28 s_0_26
        let s_0_29: Bits = s_0_28 << s_0_26;
        // C s_0_30: sub s_0_29 s_0_28
        let s_0_30: Bits = ((s_0_29) - (s_0_28));
        // D s_0_31: and s_0_25 s_0_30
        let s_0_31: Bits = ((s_0_25) & (s_0_30));
        // D s_0_32: lsl s_0_31 s_0_23
        let s_0_32: Bits = s_0_31 << s_0_23;
        // C s_0_33: lsl s_0_30 s_0_23
        let s_0_33: Bits = s_0_30 << s_0_23;
        // C s_0_34: cmpl s_0_33
        let s_0_34: Bits = !s_0_33;
        // D s_0_35: and s_0_24 s_0_34
        let s_0_35: Bits = ((s_0_24) & (s_0_34));
        // D s_0_36: or s_0_35 s_0_32
        let s_0_36: Bits = ((s_0_35) | (s_0_32));
        // D s_0_37: cast reint s_0_36 -> u32
        let s_0_37: u32 = (s_0_36.value() as u32);
        // D s_0_38: read-var d:i64
        let s_0_38: i64 = fn_state.d;
        // D s_0_39: cast zx s_0_38 -> i
        let s_0_39: i128 = (i128::try_from(s_0_38).unwrap());
        // D s_0_40: call R_set(s_0_39, s_0_37)
        let s_0_40: () = R_set(state, tracer, s_0_39, s_0_37);
        // D s_0_41: read-var d:i64
        let s_0_41: i64 = fn_state.d;
        // D s_0_42: cast zx s_0_41 -> i
        let s_0_42: i128 = (i128::try_from(s_0_41).unwrap());
        // D s_0_43: call R_read(s_0_42)
        let s_0_43: u32 = R_read(state, tracer, s_0_42);
        // C s_0_44: const #16s : i
        let s_0_44: i128 = 16;
        // D s_0_45: cast zx s_0_7 -> bv
        let s_0_45: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_46: const #1s : i64
        let s_0_46: i64 = 1;
        // C s_0_47: cast zx s_0_46 -> i
        let s_0_47: i128 = (i128::try_from(s_0_46).unwrap());
        // C s_0_48: const #7s : i
        let s_0_48: i128 = 7;
        // C s_0_49: add s_0_48 s_0_47
        let s_0_49: i128 = (s_0_48 + s_0_47);
        // D s_0_50: bit-extract s_0_45 s_0_44 s_0_49
        let s_0_50: Bits = (Bits::new(
            ((s_0_45) >> (s_0_44)).value(),
            u16::try_from(s_0_49).unwrap(),
        ));
        // D s_0_51: cast reint s_0_50 -> u8
        let s_0_51: u8 = (s_0_50.value() as u8);
        // C s_0_52: const #16s : i
        let s_0_52: i128 = 16;
        // D s_0_53: cast zx s_0_51 -> bv
        let s_0_53: Bits = Bits::new(s_0_51 as u128, 8u16);
        // D s_0_54: bits-cast sx s_0_53 -> bv length s_0_52
        let s_0_54: Bits = s_0_53.sign_extend(s_0_52);
        // D s_0_55: cast reint s_0_54 -> u16
        let s_0_55: u16 = (s_0_54.value() as u16);
        // C s_0_56: const #16s : i
        let s_0_56: i128 = 16;
        // D s_0_57: cast zx s_0_43 -> bv
        let s_0_57: Bits = Bits::new(s_0_43 as u128, 32u16);
        // D s_0_58: cast zx s_0_55 -> bv
        let s_0_58: Bits = Bits::new(s_0_55 as u128, 16u16);
        // C s_0_59: const #15s : i
        let s_0_59: i128 = 15;
        // C s_0_60: const #1u : u64
        let s_0_60: u64 = 1;
        // C s_0_61: cast zx s_0_60 -> bv
        let s_0_61: Bits = Bits::new(s_0_60 as u128, 64u16);
        // C s_0_62: lsl s_0_61 s_0_59
        let s_0_62: Bits = s_0_61 << s_0_59;
        // C s_0_63: sub s_0_62 s_0_61
        let s_0_63: Bits = ((s_0_62) - (s_0_61));
        // D s_0_64: and s_0_58 s_0_63
        let s_0_64: Bits = ((s_0_58) & (s_0_63));
        // D s_0_65: lsl s_0_64 s_0_56
        let s_0_65: Bits = s_0_64 << s_0_56;
        // C s_0_66: lsl s_0_63 s_0_56
        let s_0_66: Bits = s_0_63 << s_0_56;
        // C s_0_67: cmpl s_0_66
        let s_0_67: Bits = !s_0_66;
        // D s_0_68: and s_0_57 s_0_67
        let s_0_68: Bits = ((s_0_57) & (s_0_67));
        // D s_0_69: or s_0_68 s_0_65
        let s_0_69: Bits = ((s_0_68) | (s_0_65));
        // D s_0_70: cast reint s_0_69 -> u32
        let s_0_70: u32 = (s_0_69.value() as u32);
        // D s_0_71: read-var d:i64
        let s_0_71: i64 = fn_state.d;
        // D s_0_72: cast zx s_0_71 -> i
        let s_0_72: i128 = (i128::try_from(s_0_71).unwrap());
        // D s_0_73: call R_set(s_0_72, s_0_70)
        let s_0_73: () = R_set(state, tracer, s_0_72, s_0_70);
        // N s_0_74: return
        return;
    }
}
