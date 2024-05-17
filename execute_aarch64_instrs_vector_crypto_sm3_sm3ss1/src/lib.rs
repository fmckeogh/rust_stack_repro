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
use ROL::*;
use set_subrange_zeros::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use V_set::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3ss1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u128,
        a: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // C s_1_10: const #128s : i64
        let s_1_10: i64 = 128;
        // D s_1_11: read-var a:i64
        let s_1_11: i64 = fn_state.a;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call V_read(s_1_12, s_1_10)
        let s_1_13: Bits = V_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast reint s_1_13 -> u128
        let s_1_14: u128 = (s_1_13.value() as u128);
        // C s_1_15: const #96s : i
        let s_1_15: i128 = 96;
        // D s_1_16: cast zx s_1_9 -> bv
        let s_1_16: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_17: const #1s : i64
        let s_1_17: i64 = 1;
        // C s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: const #31s : i
        let s_1_19: i128 = 31;
        // C s_1_20: add s_1_19 s_1_18
        let s_1_20: i128 = (s_1_19 + s_1_18);
        // D s_1_21: bit-extract s_1_16 s_1_15 s_1_20
        let s_1_21: Bits = (Bits::new(
            ((s_1_16) >> (s_1_15)).value(),
            u16::try_from(s_1_20).unwrap(),
        ));
        // D s_1_22: cast reint s_1_21 -> u32
        let s_1_22: u32 = (s_1_21.value() as u32);
        // C s_1_23: const #12s : i
        let s_1_23: i128 = 12;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 32u16);
        // D s_1_25: call ROL(s_1_24, s_1_23)
        let s_1_25: Bits = ROL(state, tracer, s_1_24, s_1_23);
        // D s_1_26: cast reint s_1_25 -> u32
        let s_1_26: u32 = (s_1_25.value() as u32);
        // C s_1_27: const #96s : i
        let s_1_27: i128 = 96;
        // D s_1_28: cast zx s_1_4 -> bv
        let s_1_28: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_29: const #1s : i64
        let s_1_29: i64 = 1;
        // C s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // C s_1_31: const #31s : i
        let s_1_31: i128 = 31;
        // C s_1_32: add s_1_31 s_1_30
        let s_1_32: i128 = (s_1_31 + s_1_30);
        // D s_1_33: bit-extract s_1_28 s_1_27 s_1_32
        let s_1_33: Bits = (Bits::new(
            ((s_1_28) >> (s_1_27)).value(),
            u16::try_from(s_1_32).unwrap(),
        ));
        // D s_1_34: cast reint s_1_33 -> u32
        let s_1_34: u32 = (s_1_33.value() as u32);
        // D s_1_35: cast zx s_1_26 -> bv
        let s_1_35: Bits = Bits::new(s_1_26 as u128, 32u16);
        // D s_1_36: cast zx s_1_34 -> bv
        let s_1_36: Bits = Bits::new(s_1_34 as u128, 32u16);
        // D s_1_37: add s_1_35 s_1_36
        let s_1_37: Bits = (s_1_35 + s_1_36);
        // D s_1_38: cast reint s_1_37 -> u32
        let s_1_38: u32 = (s_1_37.value() as u32);
        // C s_1_39: const #96s : i
        let s_1_39: i128 = 96;
        // D s_1_40: cast zx s_1_14 -> bv
        let s_1_40: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_41: const #1s : i64
        let s_1_41: i64 = 1;
        // C s_1_42: cast zx s_1_41 -> i
        let s_1_42: i128 = (i128::try_from(s_1_41).unwrap());
        // C s_1_43: const #31s : i
        let s_1_43: i128 = 31;
        // C s_1_44: add s_1_43 s_1_42
        let s_1_44: i128 = (s_1_43 + s_1_42);
        // D s_1_45: bit-extract s_1_40 s_1_39 s_1_44
        let s_1_45: Bits = (Bits::new(
            ((s_1_40) >> (s_1_39)).value(),
            u16::try_from(s_1_44).unwrap(),
        ));
        // D s_1_46: cast reint s_1_45 -> u32
        let s_1_46: u32 = (s_1_45.value() as u32);
        // D s_1_47: cast zx s_1_38 -> bv
        let s_1_47: Bits = Bits::new(s_1_38 as u128, 32u16);
        // D s_1_48: cast zx s_1_46 -> bv
        let s_1_48: Bits = Bits::new(s_1_46 as u128, 32u16);
        // D s_1_49: add s_1_47 s_1_48
        let s_1_49: Bits = (s_1_47 + s_1_48);
        // D s_1_50: cast reint s_1_49 -> u32
        let s_1_50: u32 = (s_1_49.value() as u32);
        // C s_1_51: const #7s : i
        let s_1_51: i128 = 7;
        // D s_1_52: cast zx s_1_50 -> bv
        let s_1_52: Bits = Bits::new(s_1_50 as u128, 32u16);
        // D s_1_53: call ROL(s_1_52, s_1_51)
        let s_1_53: Bits = ROL(state, tracer, s_1_52, s_1_51);
        // D s_1_54: cast reint s_1_53 -> u32
        let s_1_54: u32 = (s_1_53.value() as u32);
        // C s_1_55: const #96s : i
        let s_1_55: i128 = 96;
        // D s_1_56: read-var result:u128
        let s_1_56: u128 = fn_state.result;
        // D s_1_57: cast zx s_1_56 -> bv
        let s_1_57: Bits = Bits::new(s_1_56 as u128, 128u16);
        // D s_1_58: cast zx s_1_54 -> bv
        let s_1_58: Bits = Bits::new(s_1_54 as u128, 32u16);
        // C s_1_59: const #31s : i
        let s_1_59: i128 = 31;
        // C s_1_60: const #1u : u64
        let s_1_60: u64 = 1;
        // C s_1_61: cast zx s_1_60 -> bv
        let s_1_61: Bits = Bits::new(s_1_60 as u128, 64u16);
        // C s_1_62: lsl s_1_61 s_1_59
        let s_1_62: Bits = s_1_61 << s_1_59;
        // C s_1_63: sub s_1_62 s_1_61
        let s_1_63: Bits = ((s_1_62) - (s_1_61));
        // D s_1_64: and s_1_58 s_1_63
        let s_1_64: Bits = ((s_1_58) & (s_1_63));
        // D s_1_65: lsl s_1_64 s_1_55
        let s_1_65: Bits = s_1_64 << s_1_55;
        // C s_1_66: lsl s_1_63 s_1_55
        let s_1_66: Bits = s_1_63 << s_1_55;
        // C s_1_67: cmpl s_1_66
        let s_1_67: Bits = !s_1_66;
        // D s_1_68: and s_1_57 s_1_67
        let s_1_68: Bits = ((s_1_57) & (s_1_67));
        // D s_1_69: or s_1_68 s_1_65
        let s_1_69: Bits = ((s_1_68) | (s_1_65));
        // D s_1_70: cast reint s_1_69 -> u128
        let s_1_70: u128 = (s_1_69.value() as u128);
        // D s_1_71: write-var result <= s_1_70
        fn_state.result = s_1_70;
        // C s_1_72: const #128s : i
        let s_1_72: i128 = 128;
        // C s_1_73: const #95s : i
        let s_1_73: i128 = 95;
        // C s_1_74: const #0s : i
        let s_1_74: i128 = 0;
        // D s_1_75: cast zx s_1_70 -> bv
        let s_1_75: Bits = Bits::new(s_1_70 as u128, 128u16);
        // D s_1_76: call set_subrange_zeros(s_1_72, s_1_75, s_1_73, s_1_74)
        let s_1_76: Bits = set_subrange_zeros(
            state,
            tracer,
            s_1_72,
            s_1_75,
            s_1_73,
            s_1_74,
        );
        // D s_1_77: cast reint s_1_76 -> u128
        let s_1_77: u128 = (s_1_76.value() as u128);
        // D s_1_78: write-var result <= s_1_77
        fn_state.result = s_1_77;
        // C s_1_79: const #128s : i64
        let s_1_79: i64 = 128;
        // D s_1_80: read-var d:i64
        let s_1_80: i64 = fn_state.d;
        // D s_1_81: cast zx s_1_80 -> i
        let s_1_81: i128 = (i128::try_from(s_1_80).unwrap());
        // D s_1_82: cast zx s_1_77 -> bv
        let s_1_82: Bits = Bits::new(s_1_77 as u128, 128u16);
        // D s_1_83: call V_set(s_1_81, s_1_79, s_1_82)
        let s_1_83: () = V_set(state, tracer, s_1_81, s_1_79, s_1_82);
        // N s_1_84: return
        return;
    }
}
