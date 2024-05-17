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
use Shift::*;
use AddWithCarry::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_CMP_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878764: ProductTyped54bc449dd09e5bd,
        m: i64,
        n: i64,
        shift_n: i128,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        m,
        n,
        shift_n,
        shift_t,
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
        // C s_0_3: const #16971u : u32
        let s_0_3: u32 = 16971;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: read-var shift_t:u32
        let s_0_6: u32 = fn_state.shift_t;
        // D s_0_7: read-var shift_n:i
        let s_0_7: i128 = fn_state.shift_n;
        // D s_0_8: call Shift(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: Bits = Shift(state, tracer, s_0_5, s_0_6, s_0_7, s_0_4);
        // D s_0_9: cast reint s_0_8 -> u32
        let s_0_9: u32 = (s_0_8.value() as u32);
        // D s_0_10: read-var n:i64
        let s_0_10: i64 = fn_state.n;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call R_read(s_0_11)
        let s_0_12: u32 = R_read(state, tracer, s_0_11);
        // D s_0_13: cast zx s_0_9 -> bv
        let s_0_13: Bits = Bits::new(s_0_9 as u128, 32u16);
        // D s_0_14: not s_0_13
        let s_0_14: Bits = !s_0_13;
        // D s_0_15: cast reint s_0_14 -> u32
        let s_0_15: u32 = (s_0_14.value() as u32);
        // D s_0_16: cast zx s_0_12 -> bv
        let s_0_16: Bits = Bits::new(s_0_12 as u128, 32u16);
        // D s_0_17: cast zx s_0_15 -> bv
        let s_0_17: Bits = Bits::new(s_0_15 as u128, 32u16);
        // C s_0_18: const #1u : u8
        let s_0_18: bool = true;
        // D s_0_19: call AddWithCarry(s_0_16, s_0_17, s_0_18)
        let s_0_19: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_16,
            s_0_17,
            s_0_18,
        );
        // D s_0_20: write-var gs#878764 <= s_0_19
        fn_state.gs_878764 = s_0_19;
        // D s_0_21: read-var gs#878764.1:struct
        let s_0_21: u8 = fn_state.gs_878764._1;
        // C s_0_22: const #3s : i
        let s_0_22: i128 = 3;
        // D s_0_23: cast zx s_0_21 -> bv
        let s_0_23: Bits = Bits::new(s_0_21 as u128, 4u16);
        // C s_0_24: const #1s : i64
        let s_0_24: i64 = 1;
        // C s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // C s_0_26: const #0s : i
        let s_0_26: i128 = 0;
        // C s_0_27: add s_0_26 s_0_25
        let s_0_27: i128 = (s_0_26 + s_0_25);
        // D s_0_28: bit-extract s_0_23 s_0_22 s_0_27
        let s_0_28: Bits = (Bits::new(
            ((s_0_23) >> (s_0_22)).value(),
            u16::try_from(s_0_27).unwrap(),
        ));
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: bool = ((s_0_28.value()) != 0);
        // C s_0_30: const #16984u : u32
        let s_0_30: u32 = 16984;
        // N s_0_31: write-reg s_0_30 <= s_0_29
        let s_0_31: () = {
            state.write_register::<bool>(s_0_30 as isize, s_0_29);
            tracer.write_register(s_0_30 as isize, s_0_29);
        };
        // C s_0_32: const #2s : i
        let s_0_32: i128 = 2;
        // D s_0_33: cast zx s_0_21 -> bv
        let s_0_33: Bits = Bits::new(s_0_21 as u128, 4u16);
        // C s_0_34: const #1s : i64
        let s_0_34: i64 = 1;
        // C s_0_35: cast zx s_0_34 -> i
        let s_0_35: i128 = (i128::try_from(s_0_34).unwrap());
        // C s_0_36: const #0s : i
        let s_0_36: i128 = 0;
        // C s_0_37: add s_0_36 s_0_35
        let s_0_37: i128 = (s_0_36 + s_0_35);
        // D s_0_38: bit-extract s_0_33 s_0_32 s_0_37
        let s_0_38: Bits = (Bits::new(
            ((s_0_33) >> (s_0_32)).value(),
            u16::try_from(s_0_37).unwrap(),
        ));
        // D s_0_39: cast reint s_0_38 -> u8
        let s_0_39: bool = ((s_0_38.value()) != 0);
        // C s_0_40: const #16997u : u32
        let s_0_40: u32 = 16997;
        // N s_0_41: write-reg s_0_40 <= s_0_39
        let s_0_41: () = {
            state.write_register::<bool>(s_0_40 as isize, s_0_39);
            tracer.write_register(s_0_40 as isize, s_0_39);
        };
        // C s_0_42: const #1s : i
        let s_0_42: i128 = 1;
        // D s_0_43: cast zx s_0_21 -> bv
        let s_0_43: Bits = Bits::new(s_0_21 as u128, 4u16);
        // C s_0_44: const #1s : i64
        let s_0_44: i64 = 1;
        // C s_0_45: cast zx s_0_44 -> i
        let s_0_45: i128 = (i128::try_from(s_0_44).unwrap());
        // C s_0_46: const #0s : i
        let s_0_46: i128 = 0;
        // C s_0_47: add s_0_46 s_0_45
        let s_0_47: i128 = (s_0_46 + s_0_45);
        // D s_0_48: bit-extract s_0_43 s_0_42 s_0_47
        let s_0_48: Bits = (Bits::new(
            ((s_0_43) >> (s_0_42)).value(),
            u16::try_from(s_0_47).unwrap(),
        ));
        // D s_0_49: cast reint s_0_48 -> u8
        let s_0_49: bool = ((s_0_48.value()) != 0);
        // C s_0_50: const #16971u : u32
        let s_0_50: u32 = 16971;
        // N s_0_51: write-reg s_0_50 <= s_0_49
        let s_0_51: () = {
            state.write_register::<bool>(s_0_50 as isize, s_0_49);
            tracer.write_register(s_0_50 as isize, s_0_49);
        };
        // C s_0_52: const #0s : i
        let s_0_52: i128 = 0;
        // D s_0_53: cast zx s_0_21 -> bv
        let s_0_53: Bits = Bits::new(s_0_21 as u128, 4u16);
        // C s_0_54: const #1s : i64
        let s_0_54: i64 = 1;
        // C s_0_55: cast zx s_0_54 -> i
        let s_0_55: i128 = (i128::try_from(s_0_54).unwrap());
        // C s_0_56: const #0s : i
        let s_0_56: i128 = 0;
        // C s_0_57: add s_0_56 s_0_55
        let s_0_57: i128 = (s_0_56 + s_0_55);
        // D s_0_58: bit-extract s_0_53 s_0_52 s_0_57
        let s_0_58: Bits = (Bits::new(
            ((s_0_53) >> (s_0_52)).value(),
            u16::try_from(s_0_57).unwrap(),
        ));
        // D s_0_59: cast reint s_0_58 -> u8
        let s_0_59: bool = ((s_0_58.value()) != 0);
        // C s_0_60: const #16996u : u32
        let s_0_60: u32 = 16996;
        // N s_0_61: write-reg s_0_60 <= s_0_59
        let s_0_61: () = {
            state.write_register::<bool>(s_0_60 as isize, s_0_59);
            tracer.write_register(s_0_60 as isize, s_0_59);
        };
        // N s_0_62: return
        return;
    }
}
