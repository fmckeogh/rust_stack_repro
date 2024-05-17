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
use ThisInstrAddr::*;
use IsSecureEL2Enabled::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use common::*;
pub fn AArch64_MonitorModeTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24789: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        vect_offset: i64,
        preferred_exception_return: u64,
        gs_24789: (),
    }
    let fn_state = FunctionState {
        gs_24789,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // D s_0_4: write-var preferred_exception_return <= s_0_3
        fn_state.preferred_exception_return = s_0_3;
        // C s_0_5: const #0u : u8
        let s_0_5: u8 = 0;
        // C s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // C s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // C s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var vect_offset <= s_0_8
        fn_state.vect_offset = s_0_8;
        // C s_0_10: const #0u : u32
        let s_0_10: u32 = 0;
        // S s_0_11: call ExceptionSyndrome(s_0_10)
        let s_0_11: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_0_10,
        );
        // D s_0_12: write-var except <= s_0_11
        fn_state.except = s_0_11;
        // C s_0_13: const #() : ()
        let s_0_13: () = ();
        // S s_0_14: call IsSecureEL2Enabled(s_0_13)
        let s_0_14: bool = IsSecureEL2Enabled(state, tracer, s_0_13);
        // N s_0_15: branch s_0_14 b3 b1
        if s_0_14 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var vect_offset:i64
        let s_2_0: i64 = fn_state.vect_offset;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // C s_2_2: const #424u : u32
        let s_2_2: u32 = 424;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: read-var except:struct
        let s_2_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_5: read-var preferred_exception_return:u64
        let s_2_5: u64 = fn_state.preferred_exception_return;
        // D s_2_6: call AArch64_TakeException(s_2_3, s_2_4, s_2_5, s_2_1)
        let s_2_6: () = AArch64_TakeException(state, tracer, s_2_3, s_2_4, s_2_5, s_2_1);
        // N s_2_7: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var vect_offset:i64
        let s_3_0: i64 = fn_state.vect_offset;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: read-var except:struct
        let s_3_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_5: read-var preferred_exception_return:u64
        let s_3_5: u64 = fn_state.preferred_exception_return;
        // D s_3_6: call AArch64_TakeException(s_3_3, s_3_4, s_3_5, s_3_1)
        let s_3_6: () = AArch64_TakeException(state, tracer, s_3_3, s_3_4, s_3_5, s_3_1);
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}