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
use AddWithCarry::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_CMN_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878639: ProductTyped54bc449dd09e5bd,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var imm32:u32
        let s_0_4: u32 = fn_state.imm32;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 32u16);
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // D s_0_7: call AddWithCarry(s_0_3, s_0_5, s_0_6)
        let s_0_7: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_3,
            s_0_5,
            s_0_6,
        );
        // D s_0_8: write-var gs#878639 <= s_0_7
        fn_state.gs_878639 = s_0_7;
        // D s_0_9: read-var gs#878639.1:struct
        let s_0_9: u8 = fn_state.gs_878639._1;
        // C s_0_10: const #3s : i
        let s_0_10: i128 = 3;
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_12: const #1s : i64
        let s_0_12: i64 = 1;
        // C s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // C s_0_14: const #0s : i
        let s_0_14: i128 = 0;
        // C s_0_15: add s_0_14 s_0_13
        let s_0_15: i128 = (s_0_14 + s_0_13);
        // D s_0_16: bit-extract s_0_11 s_0_10 s_0_15
        let s_0_16: Bits = (Bits::new(
            ((s_0_11) >> (s_0_10)).value(),
            u16::try_from(s_0_15).unwrap(),
        ));
        // D s_0_17: cast reint s_0_16 -> u8
        let s_0_17: bool = ((s_0_16.value()) != 0);
        // C s_0_18: const #16984u : u32
        let s_0_18: u32 = 16984;
        // N s_0_19: write-reg s_0_18 <= s_0_17
        let s_0_19: () = {
            state.write_register::<bool>(s_0_18 as isize, s_0_17);
            tracer.write_register(s_0_18 as isize, s_0_17);
        };
        // C s_0_20: const #2s : i
        let s_0_20: i128 = 2;
        // D s_0_21: cast zx s_0_9 -> bv
        let s_0_21: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_22: const #1s : i64
        let s_0_22: i64 = 1;
        // C s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // C s_0_24: const #0s : i
        let s_0_24: i128 = 0;
        // C s_0_25: add s_0_24 s_0_23
        let s_0_25: i128 = (s_0_24 + s_0_23);
        // D s_0_26: bit-extract s_0_21 s_0_20 s_0_25
        let s_0_26: Bits = (Bits::new(
            ((s_0_21) >> (s_0_20)).value(),
            u16::try_from(s_0_25).unwrap(),
        ));
        // D s_0_27: cast reint s_0_26 -> u8
        let s_0_27: bool = ((s_0_26.value()) != 0);
        // C s_0_28: const #16997u : u32
        let s_0_28: u32 = 16997;
        // N s_0_29: write-reg s_0_28 <= s_0_27
        let s_0_29: () = {
            state.write_register::<bool>(s_0_28 as isize, s_0_27);
            tracer.write_register(s_0_28 as isize, s_0_27);
        };
        // C s_0_30: const #1s : i
        let s_0_30: i128 = 1;
        // D s_0_31: cast zx s_0_9 -> bv
        let s_0_31: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_32: const #1s : i64
        let s_0_32: i64 = 1;
        // C s_0_33: cast zx s_0_32 -> i
        let s_0_33: i128 = (i128::try_from(s_0_32).unwrap());
        // C s_0_34: const #0s : i
        let s_0_34: i128 = 0;
        // C s_0_35: add s_0_34 s_0_33
        let s_0_35: i128 = (s_0_34 + s_0_33);
        // D s_0_36: bit-extract s_0_31 s_0_30 s_0_35
        let s_0_36: Bits = (Bits::new(
            ((s_0_31) >> (s_0_30)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_37: cast reint s_0_36 -> u8
        let s_0_37: bool = ((s_0_36.value()) != 0);
        // C s_0_38: const #16971u : u32
        let s_0_38: u32 = 16971;
        // N s_0_39: write-reg s_0_38 <= s_0_37
        let s_0_39: () = {
            state.write_register::<bool>(s_0_38 as isize, s_0_37);
            tracer.write_register(s_0_38 as isize, s_0_37);
        };
        // C s_0_40: const #0s : i
        let s_0_40: i128 = 0;
        // D s_0_41: cast zx s_0_9 -> bv
        let s_0_41: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_42: const #1s : i64
        let s_0_42: i64 = 1;
        // C s_0_43: cast zx s_0_42 -> i
        let s_0_43: i128 = (i128::try_from(s_0_42).unwrap());
        // C s_0_44: const #0s : i
        let s_0_44: i128 = 0;
        // C s_0_45: add s_0_44 s_0_43
        let s_0_45: i128 = (s_0_44 + s_0_43);
        // D s_0_46: bit-extract s_0_41 s_0_40 s_0_45
        let s_0_46: Bits = (Bits::new(
            ((s_0_41) >> (s_0_40)).value(),
            u16::try_from(s_0_45).unwrap(),
        ));
        // D s_0_47: cast reint s_0_46 -> u8
        let s_0_47: bool = ((s_0_46.value()) != 0);
        // C s_0_48: const #16996u : u32
        let s_0_48: u32 = 16996;
        // N s_0_49: write-reg s_0_48 <= s_0_47
        let s_0_49: () = {
            state.write_register::<bool>(s_0_48 as isize, s_0_47);
            tracer.write_register(s_0_48 as isize, s_0_47);
        };
        // N s_0_50: return
        return;
    }
}
