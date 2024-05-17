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
pub fn execute_aarch32_instrs_CMN_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878667: ProductTyped54bc449dd09e5bd,
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
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 32u16);
        // D s_0_14: cast zx s_0_9 -> bv
        let s_0_14: Bits = Bits::new(s_0_9 as u128, 32u16);
        // C s_0_15: const #0u : u8
        let s_0_15: bool = false;
        // D s_0_16: call AddWithCarry(s_0_13, s_0_14, s_0_15)
        let s_0_16: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_13,
            s_0_14,
            s_0_15,
        );
        // D s_0_17: write-var gs#878667 <= s_0_16
        fn_state.gs_878667 = s_0_16;
        // D s_0_18: read-var gs#878667.1:struct
        let s_0_18: u8 = fn_state.gs_878667._1;
        // C s_0_19: const #3s : i
        let s_0_19: i128 = 3;
        // D s_0_20: cast zx s_0_18 -> bv
        let s_0_20: Bits = Bits::new(s_0_18 as u128, 4u16);
        // C s_0_21: const #1s : i64
        let s_0_21: i64 = 1;
        // C s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // C s_0_24: add s_0_23 s_0_22
        let s_0_24: i128 = (s_0_23 + s_0_22);
        // D s_0_25: bit-extract s_0_20 s_0_19 s_0_24
        let s_0_25: Bits = (Bits::new(
            ((s_0_20) >> (s_0_19)).value(),
            u16::try_from(s_0_24).unwrap(),
        ));
        // D s_0_26: cast reint s_0_25 -> u8
        let s_0_26: bool = ((s_0_25.value()) != 0);
        // C s_0_27: const #16984u : u32
        let s_0_27: u32 = 16984;
        // N s_0_28: write-reg s_0_27 <= s_0_26
        let s_0_28: () = {
            state.write_register::<bool>(s_0_27 as isize, s_0_26);
            tracer.write_register(s_0_27 as isize, s_0_26);
        };
        // C s_0_29: const #2s : i
        let s_0_29: i128 = 2;
        // D s_0_30: cast zx s_0_18 -> bv
        let s_0_30: Bits = Bits::new(s_0_18 as u128, 4u16);
        // C s_0_31: const #1s : i64
        let s_0_31: i64 = 1;
        // C s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // C s_0_33: const #0s : i
        let s_0_33: i128 = 0;
        // C s_0_34: add s_0_33 s_0_32
        let s_0_34: i128 = (s_0_33 + s_0_32);
        // D s_0_35: bit-extract s_0_30 s_0_29 s_0_34
        let s_0_35: Bits = (Bits::new(
            ((s_0_30) >> (s_0_29)).value(),
            u16::try_from(s_0_34).unwrap(),
        ));
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: bool = ((s_0_35.value()) != 0);
        // C s_0_37: const #16997u : u32
        let s_0_37: u32 = 16997;
        // N s_0_38: write-reg s_0_37 <= s_0_36
        let s_0_38: () = {
            state.write_register::<bool>(s_0_37 as isize, s_0_36);
            tracer.write_register(s_0_37 as isize, s_0_36);
        };
        // C s_0_39: const #1s : i
        let s_0_39: i128 = 1;
        // D s_0_40: cast zx s_0_18 -> bv
        let s_0_40: Bits = Bits::new(s_0_18 as u128, 4u16);
        // C s_0_41: const #1s : i64
        let s_0_41: i64 = 1;
        // C s_0_42: cast zx s_0_41 -> i
        let s_0_42: i128 = (i128::try_from(s_0_41).unwrap());
        // C s_0_43: const #0s : i
        let s_0_43: i128 = 0;
        // C s_0_44: add s_0_43 s_0_42
        let s_0_44: i128 = (s_0_43 + s_0_42);
        // D s_0_45: bit-extract s_0_40 s_0_39 s_0_44
        let s_0_45: Bits = (Bits::new(
            ((s_0_40) >> (s_0_39)).value(),
            u16::try_from(s_0_44).unwrap(),
        ));
        // D s_0_46: cast reint s_0_45 -> u8
        let s_0_46: bool = ((s_0_45.value()) != 0);
        // C s_0_47: const #16971u : u32
        let s_0_47: u32 = 16971;
        // N s_0_48: write-reg s_0_47 <= s_0_46
        let s_0_48: () = {
            state.write_register::<bool>(s_0_47 as isize, s_0_46);
            tracer.write_register(s_0_47 as isize, s_0_46);
        };
        // C s_0_49: const #0s : i
        let s_0_49: i128 = 0;
        // D s_0_50: cast zx s_0_18 -> bv
        let s_0_50: Bits = Bits::new(s_0_18 as u128, 4u16);
        // C s_0_51: const #1s : i64
        let s_0_51: i64 = 1;
        // C s_0_52: cast zx s_0_51 -> i
        let s_0_52: i128 = (i128::try_from(s_0_51).unwrap());
        // C s_0_53: const #0s : i
        let s_0_53: i128 = 0;
        // C s_0_54: add s_0_53 s_0_52
        let s_0_54: i128 = (s_0_53 + s_0_52);
        // D s_0_55: bit-extract s_0_50 s_0_49 s_0_54
        let s_0_55: Bits = (Bits::new(
            ((s_0_50) >> (s_0_49)).value(),
            u16::try_from(s_0_54).unwrap(),
        ));
        // D s_0_56: cast reint s_0_55 -> u8
        let s_0_56: bool = ((s_0_55.value()) != 0);
        // C s_0_57: const #16996u : u32
        let s_0_57: u32 = 16996;
        // N s_0_58: write-reg s_0_57 <= s_0_56
        let s_0_58: () = {
            state.write_register::<bool>(s_0_57 as isize, s_0_56);
            tracer.write_register(s_0_57 as isize, s_0_56);
        };
        // N s_0_59: return
        return;
    }
}
