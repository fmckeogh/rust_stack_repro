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
pub fn execute_aarch32_instrs_TEQ_rr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    s: i64,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_887326: ProductTypef506aa96a892fe52,
        m: i64,
        n: i64,
        s: i64,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        m,
        n,
        s,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var s:i64
        let s_0_0: i64 = fn_state.s;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 8u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call R_read(s_0_15)
        let s_0_16: u32 = R_read(state, tracer, s_0_15);
        // C s_0_17: const #16971u : u32
        let s_0_17: u32 = 16971;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: bool = {
            let value = state.read_register::<bool>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_16 -> bv
        let s_0_19: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_20: cast zx s_0_13 -> i
        let s_0_20: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_21: read-var shift_t:u32
        let s_0_21: u32 = fn_state.shift_t;
        // D s_0_22: call Shift_C(s_0_19, s_0_21, s_0_20, s_0_18)
        let s_0_22: ProductTypef506aa96a892fe52 = Shift_C(
            state,
            tracer,
            s_0_19,
            s_0_21,
            s_0_20,
            s_0_18,
        );
        // D s_0_23: write-var gs#887326 <= s_0_22
        fn_state.gs_887326 = s_0_22;
        // D s_0_24: read-var gs#887326.0:struct
        let s_0_24: Bits = fn_state.gs_887326._0;
        // D s_0_25: cast reint s_0_24 -> u32
        let s_0_25: u32 = (s_0_24.value() as u32);
        // D s_0_26: read-var gs#887326.1:struct
        let s_0_26: bool = fn_state.gs_887326._1;
        // D s_0_27: read-var n:i64
        let s_0_27: i64 = fn_state.n;
        // D s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_29: call R_read(s_0_28)
        let s_0_29: u32 = R_read(state, tracer, s_0_28);
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 32u16);
        // D s_0_31: cast zx s_0_25 -> bv
        let s_0_31: Bits = Bits::new(s_0_25 as u128, 32u16);
        // D s_0_32: xor s_0_30 s_0_31
        let s_0_32: Bits = ((s_0_30) ^ (s_0_31));
        // D s_0_33: cast reint s_0_32 -> u32
        let s_0_33: u32 = (s_0_32.value() as u32);
        // C s_0_34: const #31s : i
        let s_0_34: i128 = 31;
        // D s_0_35: cast zx s_0_33 -> bv
        let s_0_35: Bits = Bits::new(s_0_33 as u128, 32u16);
        // C s_0_36: const #1u : u64
        let s_0_36: u64 = 1;
        // D s_0_37: bit-extract s_0_35 s_0_34 s_0_36
        let s_0_37: Bits = (Bits::new(
            ((s_0_35) >> (s_0_34)).value(),
            u16::try_from(s_0_36).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u8
        let s_0_38: bool = ((s_0_37.value()) != 0);
        // C s_0_39: const #0s : i
        let s_0_39: i128 = 0;
        // C s_0_40: const #0u : u64
        let s_0_40: u64 = 0;
        // D s_0_41: cast zx s_0_38 -> u64
        let s_0_41: u64 = (s_0_38 as u64);
        // C s_0_42: const #1u : u64
        let s_0_42: u64 = 1;
        // D s_0_43: and s_0_41 s_0_42
        let s_0_43: u64 = ((s_0_41) & (s_0_42));
        // D s_0_44: cmp-eq s_0_43 s_0_42
        let s_0_44: bool = ((s_0_43) == (s_0_42));
        // D s_0_45: lsl s_0_41 s_0_39
        let s_0_45: u64 = s_0_41 << s_0_39;
        // D s_0_46: or s_0_40 s_0_45
        let s_0_46: u64 = ((s_0_40) | (s_0_45));
        // D s_0_47: cmpl s_0_45
        let s_0_47: u64 = !s_0_45;
        // D s_0_48: and s_0_40 s_0_47
        let s_0_48: u64 = ((s_0_40) & (s_0_47));
        // D s_0_49: select s_0_44 s_0_46 s_0_48
        let s_0_49: u64 = if s_0_44 { s_0_46 } else { s_0_48 };
        // D s_0_50: cast trunc s_0_49 -> u8
        let s_0_50: bool = ((s_0_49) != 0);
        // C s_0_51: const #16984u : u32
        let s_0_51: u32 = 16984;
        // N s_0_52: write-reg s_0_51 <= s_0_50
        let s_0_52: () = {
            state.write_register::<bool>(s_0_51 as isize, s_0_50);
            tracer.write_register(s_0_51 as isize, s_0_50);
        };
        // D s_0_53: cast zx s_0_33 -> bv
        let s_0_53: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_54: call IsZeroBit(s_0_53)
        let s_0_54: bool = IsZeroBit(state, tracer, s_0_53);
        // C s_0_55: const #16997u : u32
        let s_0_55: u32 = 16997;
        // N s_0_56: write-reg s_0_55 <= s_0_54
        let s_0_56: () = {
            state.write_register::<bool>(s_0_55 as isize, s_0_54);
            tracer.write_register(s_0_55 as isize, s_0_54);
        };
        // C s_0_57: const #16971u : u32
        let s_0_57: u32 = 16971;
        // N s_0_58: write-reg s_0_57 <= s_0_26
        let s_0_58: () = {
            state.write_register::<bool>(s_0_57 as isize, s_0_26);
            tracer.write_register(s_0_57 as isize, s_0_26);
        };
        // N s_0_59: return
        return;
    }
}
