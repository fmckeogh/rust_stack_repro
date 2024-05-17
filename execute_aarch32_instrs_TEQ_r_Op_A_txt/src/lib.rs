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
use IsZeroBit::*;
use Shift_C::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_TEQ_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_887296: ProductTypef506aa96a892fe52,
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
        // D s_0_8: call Shift_C(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: ProductTypef506aa96a892fe52 = Shift_C(
            state,
            tracer,
            s_0_5,
            s_0_6,
            s_0_7,
            s_0_4,
        );
        // D s_0_9: write-var gs#887296 <= s_0_8
        fn_state.gs_887296 = s_0_8;
        // D s_0_10: read-var gs#887296.0:struct
        let s_0_10: Bits = fn_state.gs_887296._0;
        // D s_0_11: cast reint s_0_10 -> u32
        let s_0_11: u32 = (s_0_10.value() as u32);
        // D s_0_12: read-var gs#887296.1:struct
        let s_0_12: bool = fn_state.gs_887296._1;
        // D s_0_13: read-var n:i64
        let s_0_13: i64 = fn_state.n;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: call R_read(s_0_14)
        let s_0_15: u32 = R_read(state, tracer, s_0_14);
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 32u16);
        // D s_0_17: cast zx s_0_11 -> bv
        let s_0_17: Bits = Bits::new(s_0_11 as u128, 32u16);
        // D s_0_18: xor s_0_16 s_0_17
        let s_0_18: Bits = ((s_0_16) ^ (s_0_17));
        // D s_0_19: cast reint s_0_18 -> u32
        let s_0_19: u32 = (s_0_18.value() as u32);
        // C s_0_20: const #31s : i
        let s_0_20: i128 = 31;
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 32u16);
        // C s_0_22: const #1u : u64
        let s_0_22: u64 = 1;
        // D s_0_23: bit-extract s_0_21 s_0_20 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_21) >> (s_0_20)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: bool = ((s_0_23.value()) != 0);
        // C s_0_25: const #0s : i
        let s_0_25: i128 = 0;
        // C s_0_26: const #0u : u64
        let s_0_26: u64 = 0;
        // D s_0_27: cast zx s_0_24 -> u64
        let s_0_27: u64 = (s_0_24 as u64);
        // C s_0_28: const #1u : u64
        let s_0_28: u64 = 1;
        // D s_0_29: and s_0_27 s_0_28
        let s_0_29: u64 = ((s_0_27) & (s_0_28));
        // D s_0_30: cmp-eq s_0_29 s_0_28
        let s_0_30: bool = ((s_0_29) == (s_0_28));
        // D s_0_31: lsl s_0_27 s_0_25
        let s_0_31: u64 = s_0_27 << s_0_25;
        // D s_0_32: or s_0_26 s_0_31
        let s_0_32: u64 = ((s_0_26) | (s_0_31));
        // D s_0_33: cmpl s_0_31
        let s_0_33: u64 = !s_0_31;
        // D s_0_34: and s_0_26 s_0_33
        let s_0_34: u64 = ((s_0_26) & (s_0_33));
        // D s_0_35: select s_0_30 s_0_32 s_0_34
        let s_0_35: u64 = if s_0_30 { s_0_32 } else { s_0_34 };
        // D s_0_36: cast trunc s_0_35 -> u8
        let s_0_36: bool = ((s_0_35) != 0);
        // C s_0_37: const #16984u : u32
        let s_0_37: u32 = 16984;
        // N s_0_38: write-reg s_0_37 <= s_0_36
        let s_0_38: () = {
            state.write_register::<bool>(s_0_37 as isize, s_0_36);
            tracer.write_register(s_0_37 as isize, s_0_36);
        };
        // D s_0_39: cast zx s_0_19 -> bv
        let s_0_39: Bits = Bits::new(s_0_19 as u128, 32u16);
        // D s_0_40: call IsZeroBit(s_0_39)
        let s_0_40: bool = IsZeroBit(state, tracer, s_0_39);
        // C s_0_41: const #16997u : u32
        let s_0_41: u32 = 16997;
        // N s_0_42: write-reg s_0_41 <= s_0_40
        let s_0_42: () = {
            state.write_register::<bool>(s_0_41 as isize, s_0_40);
            tracer.write_register(s_0_41 as isize, s_0_40);
        };
        // C s_0_43: const #16971u : u32
        let s_0_43: u32 = 16971;
        // N s_0_44: write-reg s_0_43 <= s_0_12
        let s_0_44: () = {
            state.write_register::<bool>(s_0_43 as isize, s_0_12);
            tracer.write_register(s_0_43 as isize, s_0_12);
        };
        // N s_0_45: return
        return;
    }
}
